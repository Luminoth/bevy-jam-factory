// How to use this:
//   You should copy/paste this into your project and use it much like examples/tiles.rs uses this
//   file. When you do so you will need to adjust the code based on whether you're using the
//   'atlas` feature in bevy_ecs_tilemap. The bevy_ecs_tilemap uses this as an example of how to
//   use both single image tilesets and image collection tilesets. Since your project won't have
//   the 'atlas' feature defined in your Cargo config, the expressions prefixed by the #[cfg(...)]
//   macro will not compile in your project as-is. If your project depends on the bevy_ecs_tilemap
//   'atlas' feature then move all of the expressions prefixed by #[cfg(not(feature = "atlas"))].
//   Otherwise remove all of the expressions prefixed by #[cfg(feature = "atlas")].
//
// Functional limitations:
//   * When the 'atlas' feature is enabled tilesets using a collection of images will be skipped.
//   * Only finite tile layers are loaded. Infinite tile layers and object layers will be skipped.

use std::collections::HashMap;
use std::io::{Cursor, Error, ErrorKind, Read};
use std::path::Path;
use std::sync::Arc;

use bevy::{
    asset::{io::Reader, AssetLoader, AssetPath, AsyncReadExt},
    prelude::*,
    reflect::TypePath,
};
use bevy_ecs_tilemap::prelude::*;
use thiserror::Error;

#[derive(TypePath, Asset)]
pub struct TiledMap {
    pub map: tiled::Map,

    // TODO: storing by name is pretty bad here
    // but we need a way to from a Tileset to this texture
    // and I can't find an easier id for it (so far)
    pub tilemap_textures: HashMap<String, TilemapTexture>,

    // The offset into the tileset_images for each tile id within each tileset.
    // TODO: storing by name is pretty bad here
    // but we need a way to from a Tileset to this texture
    // and I can't find an easier id for it (so far)
    #[cfg(not(feature = "atlas"))]
    pub tile_image_offsets: HashMap<(String, tiled::TileId), u32>,
}

struct BytesResourceReader<'a, 'ctx> {
    bytes: Arc<[u8]>,
    load_context: &'a mut bevy::asset::LoadContext<'ctx>,
}

impl<'a, 'ctx> BytesResourceReader<'a, 'ctx> {
    fn new(bytes: &[u8], load_context: &'a mut bevy::asset::LoadContext<'ctx>) -> Self {
        Self {
            bytes: Arc::from(bytes),
            load_context,
        }
    }
}

impl<'a, 'ctx> tiled::ResourceReader for BytesResourceReader<'a, 'ctx> {
    type Resource = Box<dyn Read + 'a>;
    type Error = std::io::Error;

    fn read_from(&mut self, path: &Path) -> std::result::Result<Self::Resource, Self::Error> {
        if let Some(extension) = path.extension() {
            // TSX support adapted from https://github.com/StarArawn/bevy_ecs_tilemap/pull/429
            if extension == "tsx" {
                let future = self.load_context.read_asset_bytes(path.to_owned());
                let data = futures_lite::future::block_on(future)
                    .map_err(|err| Error::new(ErrorKind::NotFound, err))?;
                return Ok(Box::new(Cursor::new(data.clone())));
            }
        }
        Ok(Box::new(Cursor::new(self.bytes.clone())))
    }
}

pub struct TiledLoader;

#[derive(Debug, Error)]
pub enum TiledAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load Tiled file: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for TiledLoader {
    type Asset = TiledMap;
    type Settings = ();
    type Error = TiledAssetLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let path = load_context.path().to_owned();
        info!("Loading map: {}", path.display());

        let mut loader = tiled::Loader::with_cache_and_reader(
            tiled::DefaultResourceCache::new(),
            BytesResourceReader::new(&bytes, load_context),
        );
        let map = loader.load_tmx_map(&path).map_err(|e| {
            std::io::Error::new(ErrorKind::Other, format!("Could not load TMX map: {e}"))
        })?;

        let mut tilemap_textures = HashMap::default();
        #[cfg(not(feature = "atlas"))]
        let mut tile_image_offsets = HashMap::default();

        for (tileset_index, tileset) in map.tilesets().iter().enumerate() {
            let tilemap_texture = match &tileset.image {
                None => {
                    #[cfg(feature = "atlas")]
                    {
                        info!("Skipping image collection tileset '{}' which is incompatible with atlas feature", tileset.name);
                        continue;
                    }

                    #[cfg(not(feature = "atlas"))]
                    {
                        let mut tile_images: Vec<Handle<Image>> = Vec::new();
                        for (tile_id, tile) in tileset.tiles() {
                            if let Some(img) = &tile.image {
                                // The load context path is the TMX file itself. If the file is at the root of the
                                // assets/ directory structure then the tmx_dir will be empty, which is fine.
                                let tmx_dir = load_context
                                    .path()
                                    .parent()
                                    .expect("The asset load context was empty.");
                                let tile_path = tmx_dir.join(&img.source);
                                let asset_path = AssetPath::from(tile_path);
                                info!("Loading tile image from {asset_path:?} as image ({tileset_index}, {tile_id})");
                                let texture: Handle<Image> = load_context.load(asset_path.clone());
                                tile_image_offsets.insert(
                                    (tileset.name.clone(), tile_id),
                                    tile_images.len() as u32,
                                );
                                tile_images.push(texture.clone());
                            }
                        }

                        TilemapTexture::Vector(tile_images)
                    }
                }
                Some(img) => {
                    // The load context path is the TMX file itself. If the file is at the root of the
                    // assets/ directory structure then the tmx_dir will be empty, which is fine.
                    let tmx_dir = load_context
                        .path()
                        .parent()
                        .expect("The asset load context was empty.");
                    let tile_path = tmx_dir.join(&img.source);
                    let asset_path = AssetPath::from(tile_path);
                    let texture: Handle<Image> = load_context.load(asset_path.clone());

                    TilemapTexture::Single(texture.clone())
                }
            };

            tilemap_textures.insert(tileset.name.clone(), tilemap_texture);
        }

        let asset_map = TiledMap {
            map,
            tilemap_textures,
            #[cfg(not(feature = "atlas"))]
            tile_image_offsets,
        };

        info!("Loaded map: {}", path.display());
        Ok(asset_map)
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}
