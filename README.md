# Bevy Jam 5 - Factory

## Tools

* Tiled Editor - https://www.mapeditor.org/
  * https://doc.mapeditor.org/en/stable/manual/introduction/
* LDtk - https://ldtk.io/
* Bevy ECS Tilemap - https://crates.io/crates/bevy_ecs_tilemap

## Tilemap

* Tutorials
  * https://www.youtube.com/watch?v=V7VjId-77BM
  * https://www.youtube.com/watch?v=eoO5s3-__sM
  * 256x256 document to place tiles onto (this is the TileMap)
    * 32x32 grid
    * Turn on Snap to Grid
  * 400x400 starting tile
    * Shift+W is wraparound mode
    * Ctrl+A is select all
    * 50x50 grid
    * Use Layer Groups for each tile
    * Can edit brush sharpness settings to be more pixely
    * Can add Filter Mask - Map - Palletize
    * .. Transition textures
  * Copy Paste tile layers onto tile map
    * Transform to 32x32
    * Move to align with grid
  * Export tilemap to PNG

## References

* https://github.com/bevyengine/bevy/blob/main/examples/state/sub_states.rs
