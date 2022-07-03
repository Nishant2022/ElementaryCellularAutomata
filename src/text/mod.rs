use bevy::prelude::*;
use crate::automaton::events::RuleChangeEvent;

// region:      Components

#[derive(Component)]
struct TextComponent;

// endregion:   Components
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_text_system)
            .add_system(update_rule_text_system);
    }
}

// Create Nishant's Boids text
fn setup_text_system(mut commands: Commands, asset_server: Res<AssetServer>) {    
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "Rule: 30",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Medium.ttf"),
                    font_size: 35.0,
                    color: Color::rgba(1.0, 0.5, 0.5, 1.0),
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(TextComponent);
}

fn update_rule_text_system (
    mut rule_changed: EventReader<RuleChangeEvent>,
    mut query: Query<&mut Text>,
) {
    for rule in rule_changed.iter() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Rule: {}", rule.new_rule);
        }
    }
}