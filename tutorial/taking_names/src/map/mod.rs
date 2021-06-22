pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 60;
const TILES: usize = WIDTH * HEIGHT;
pub const NUM_LAYERS: usize = 4;

mod tile;
use tile::Tile;
pub use tile::TileType;
mod layer;
use layer::Layer;
mod map;
pub use map::Map;
mod layerbuilder;