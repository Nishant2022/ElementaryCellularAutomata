use bevy::prelude::Component;

use super::enums::CellState;

// region:      Components

#[derive(Clone, Copy, Component)]
pub struct Cell {
    pub state: CellState,
    pub position_x: u32,
    pub position_y: u32,
}

#[derive(Component)]
pub struct CellLabel;

// endregion:   Components