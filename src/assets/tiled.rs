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
    pub name: String,

    pub map: tiled::Map,

    // maps Tileset name to TilemapTexture
    pub tilemap_textures: HashMap<String, TilemapTexture>,
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
        for tileset in map.tilesets() {
            let tilemap_image = tileset.image.as_ref().unwrap_or_else(|| {
                panic!(
                    "Image collection in tileset '{}' is incompatible with atlas feature",
                    tileset.name
                )
            });

            // https://github.com/StarArawn/bevy_ecs_tilemap/pull/525
            let asset_path = AssetPath::from(tilemap_image.source.clone());
            let texture = load_context.load(asset_path.clone());

            let tilemap_texture = TilemapTexture::Single(texture.clone());
            tilemap_textures.insert(tileset.name.clone(), tilemap_texture);
        }

        let asset_map = TiledMap {
            name: path.display().to_string(),
            map,
            tilemap_textures,
        };

        info!("Loaded map: {}", path.display());
        Ok(asset_map)
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}
