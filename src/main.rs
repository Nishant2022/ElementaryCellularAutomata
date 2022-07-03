use automaton::AutomataPlugin;
use bevy::prelude::*;
use text::TextPlugin;

mod automaton;
mod text;

// section:      resouces

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

// endsection:   resouces
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: String::from("Nishant's Elementary Cellular Automaton"),
            width: 640.,
            height: 360.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AutomataPlugin)
        .add_plugin(TextPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>
) {

    // Add camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Add WinSize resource
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize{w: win_w, h: win_h};
    commands.insert_resource(win_size);

}