mod components;
mod data;
mod mine;
mod systems;
mod textures;

pub use components::*;
pub use data::*;
pub use mine::*;
pub use systems::*;
pub use textures::*;

pub const BACKGROUND_COLOR: [f32; 4] = [117.0 / 255.0, 117.0 / 255.0, 117.0 / 255.0, 1.0];
pub const CELL_ROW: usize = 12;
pub const CELL_COL: usize = 10;
pub const CELL_WIDTH: u32 = 25;
pub const CELL_HEIGHT: u32 = 25;
pub const HEADER_HEIGHT: u32 = 40;
pub const BTN_WIDTH: u32 = 20;
pub const BTN_HEIGHT: u32 = 20;
pub const NUMBER_WIDTH: u32 = 15;
pub const NUMBER_HEIGHT: u32 = 25;
pub const MINE_COUNT: u32 = 15;
pub const FLAG_INTERVAL: f32 = 0.4;
