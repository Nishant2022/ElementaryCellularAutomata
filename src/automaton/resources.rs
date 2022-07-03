
use bevy::prelude::Color;

use super::components::Cell;

// region:      Resources

pub struct CellSettings {
    pub cell_size: f32,
    pub num_cells: u32,
    pub dead_color: Color,
    pub alive_color: Color,
    pub rule_num: u8,
    pub rule: [bool; 8],
    pub random: bool,
}

pub struct CellGrid {
    pub grid: Vec<Vec<Cell>>,
}

pub struct RuleChanged {
    pub updated_cell_grid: bool,
    pub updated_sprites: bool,
}

// endregion:   Resources