# Bevy Jam 5 - Factory

## Design Notes

* Tile layers
  * Only support Orthogonal Orientation maps
  * 32x32 tiles
  * Tilemaps should be larger than 25x25 tiles

### Tile Properties

* Buildable: bool

### Object Properties

* ObjectType
  * Resources

## Tools

* Tiled Editor - https://www.mapeditor.org/
  * https://doc.mapeditor.org/en/stable/manual/introduction/
* Bevy ECS Tilemap - https://crates.io/crates/bevy_ecs_tilemap

## Tilemap Notes

* Tutorials
  * https://www.youtube.com/watch?v=V7VjId-77BM
  * https://www.youtube.com/watch?v=eoO5s3-__sM
* Process
  * 256x256 document to place tiles onto (this is the TileMap)
    * 32x32 grid
    * Turn on Snap to Grid
  * 400x400 starting tile
    * Shift+W is wraparound mode
    * Ctrl+A is select all
    * 50x50 grid
    * Use Layer Groups for each tile
    * Can edit brush sharpness settings to be more pixely
      * This works better with an actual 32x32 tile
    * Can add Filter Mask - Map - Palletize
      * This works better with an actual 32x32 tile
    * .. Transition textures
  * Copy Paste tile layers onto tile map
    * Transform to 32x32
    * Move to align with grid
  * Export tilemap to PNG

## Tiled Notes

* Set to "Insert Tile" mode to place Objects
  * "Select Object" mode is used to select and modify Objects
* Only single image tilesets are supported ("atlas" feature is on in bevy_ecs_tilemap)
* Only finite tile layers are supported

## References

* https://github.com/bevyengine/bevy/blob/main/examples/state/sub_states.rs
