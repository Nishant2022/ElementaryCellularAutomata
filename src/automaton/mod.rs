use bevy::prelude::*;

use self::{systems::{cell_spawn_system, update_cell_grid_system, color_grid_system, mouse_button_input_system, key_press_system, mouse_scroll_system, window_resize_system}, events::RuleChangeEvent};

mod components;
mod enums;
mod resources;
mod systems;
mod events;
pub struct AutomataPlugin;

impl Plugin for AutomataPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup system
            .add_startup_system_to_stage(StartupStage::PostStartup, cell_spawn_system)

            // Input Systems
            .add_system_set(
                SystemSet::new()
                    .label("input")
                    .with_system(mouse_button_input_system)
                    .with_system(key_press_system)
                    .with_system(mouse_scroll_system)
                    .with_system(window_resize_system)
            )

            // Cell Update systems
            .add_system_set(
                SystemSet::new()
                    .label("cell_update")
                    .with_system(update_cell_grid_system.label("update_cell_grid"))
                    .with_system(color_grid_system.after("update_cell_grid"))
            )

            // Events
            .add_event::<RuleChangeEvent>();

        #[cfg(target_arch = "wasm32")]
        {
            app.add_plugin(bevy_web_resizer::Plugin);
        }
    }
}

