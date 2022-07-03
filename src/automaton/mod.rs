use bevy::prelude::*;

use self::systems::{cell_spawn_system, update_cell_grid_system, color_grid_system, mouse_button_input_system, key_press_system, mouse_scroll_system};

mod components;
mod enums;
mod resources;
mod systems;

pub struct AutomataPlugin;

impl Plugin for AutomataPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, cell_spawn_system)
            .add_system(update_cell_grid_system)
            .add_system(color_grid_system.after(update_cell_grid_system))
            .add_system(mouse_button_input_system)
            .add_system(key_press_system)
            .add_system(mouse_scroll_system);

        #[cfg(target_arch = "wasm32")]
        {
            app.add_plugin(bevy_web_resizer::Plugin);
        }
    }
}

