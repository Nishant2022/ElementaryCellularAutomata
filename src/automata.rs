use bevy::prelude::*;
use crate::WinSize;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}
#[derive(Clone, Copy, Component)]
pub struct Cell {
    state: CellState,
    position_x: u32,
    position_y: u32,
}

#[derive(Component)]
struct CellLabel;

struct CellSettings {
    cell_size: f32,
    num_cells: u32,
    dead_color: Color,
    alive_color: Color,
    rule_num: u8,
    rule: [bool; 8],
}

struct CellGrid {
    grid: Vec<Vec<Cell>>,
}

pub struct AutomataPlugin;

impl Plugin for AutomataPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup ,cell_spawn_system)
            .add_system(update_cell_grid_system)
            .add_system(color_grid_system)
            .add_system(mouse_button_input_system)
            .add_system(key_press_system);
    }
}

fn cell_spawn_system(mut commands: Commands, win_size: Res<WinSize>) {

    // num_cells defines the length and width of cell grid
    // should optimally be a value 2^n + 1
    const NUM_CELLS: usize = 257;

    // cell_size is the size of each cell
    let cell_size: f32 = win_size.w / NUM_CELLS as f32 * 2.0;
    
    // init_rule is the initial rule that will be calculated
    let init_rule: u8 = 30;

    // Insert CellSettings resources
    commands.insert_resource(CellSettings {
        cell_size: cell_size,
        num_cells: NUM_CELLS as u32,
        dead_color: Color::BLACK,
        alive_color: Color::WHITE,
        rule_num: init_rule,
        rule: get_rule(init_rule),
    });

    // Create cell grid
    let mut cell_grid = CellGrid {grid: Vec::new()};

    // Initialize Cell Grid with grid of dead cells
    for i in 0..NUM_CELLS {
        cell_grid.grid.push(Vec::new());
        for j in 0..NUM_CELLS {
            let x_pos =  -win_size.w / 1.0 + cell_size * i as f32;
            let y_pos =  win_size.h / 1.0 - cell_size * j as f32;

            let new_cell = Cell {
                state: CellState::Dead,
                position_x: i as u32,
                position_y: j as u32,
            };

            cell_grid.grid[i].push(new_cell.clone());

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(cell_size, cell_size)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x_pos, y_pos, 10.0),
                    ..Default::default()
                },
                ..Default::default()
            })
                .insert(new_cell);

            
        }
    }
    commands.insert_resource(cell_grid);
}

// System used to update CellGrid resouce 
fn update_cell_grid_system(mut cell_grid: ResMut<CellGrid>, cell_settings: Res<CellSettings>) {
    let num_cells = cell_settings.num_cells;
   
    // Go through first row of cells and set them to Dead
    for i in 0..num_cells {
        cell_grid.grid[i as usize][0].state = CellState::Dead;
    }

    // Set middle cell of first row to Alive
    cell_grid.grid[(num_cells / 2) as usize][0].state = CellState::Alive;

    // Go through each cell, row by row, skipping the first row,
    // and determine whether a cell should be alive or dead given a rule
    for j in 1..num_cells as usize {
        for i in 0..num_cells as usize {

            // Counter is used to index into the rule vec
            let mut counter = 0;

            if i != 0 && cell_grid.grid[i - 1][j - 1].state == CellState::Alive {
                counter += 4;    
            }
            
            if i != (num_cells - 1) as usize && cell_grid.grid[i + 1][j - 1].state == CellState::Alive {
                counter += 2;
            }
            
            if cell_grid.grid[i][j - 1].state == CellState::Alive {
                counter += 1;   
            }

            // If the set rule states a cell should be alive, make it alive
            // Otherwise make it dead
            if cell_settings.rule[counter as usize] { cell_grid.grid[i][j].state = CellState::Alive}
            else { cell_grid.grid[i][j].state = CellState::Dead }

        }
    }
}

// System used to update sprites based on CellGrid Resource
fn color_grid_system(mut query: Query<(&mut Sprite, &mut Cell)>, cell_grid: Res<CellGrid>, cell_settings: Res<CellSettings>) {
    
    // Iterate through all cells
    for (mut sprite, mut cell) in query.iter_mut() {
        cell.state = cell_grid.grid[cell.position_x as usize][cell.position_y as usize].state;
        match cell.state {
            CellState::Alive => sprite.color = cell_settings.alive_color,
            CellState::Dead => sprite.color = cell_settings.dead_color,
        }
    }
}

// Used to calculate a rule array based on a given u8 value
fn get_rule(mut rule_num: u8) -> [bool; 8] {
    let mut rule: [bool; 8] = [false; 8];

    for i in 1..=8 {
        if rule_num / u8::pow(2, 8 - i) > 0 {
            rule_num -= u8::pow(2, 8 - i); 
            rule[8 - i as usize] = true;
        }
    }

    return rule;
}

// Updates rule based on mouse input
fn mouse_button_input_system (
    buttons: Res<Input<MouseButton>>, 
    mut cell_settings: ResMut<CellSettings>,
) {
    // Right mouse button decreases rule number
    if buttons.just_pressed(MouseButton::Right) {
        cell_settings.rule_num -= 1;
        cell_settings.rule = get_rule(cell_settings.rule_num);
    }
    
    // Left mouse button increases rule number
    if buttons.just_pressed(MouseButton::Left) {
        cell_settings.rule_num += 1;
        cell_settings.rule = get_rule(cell_settings.rule_num);
    }
}

// Update cell positions based on keyboard input
fn key_press_system (
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Sprite), With<Cell>>,
    mut settings: ResMut<CellSettings>,
) {
    // W key moves cells up
    if keys.pressed(KeyCode::W) {
        for (mut transform, _sprite) in query.iter_mut() {
            transform.translation.y -= 0.5 * settings.cell_size;
        }
    }
    
    // S key moves cells down
    if keys.pressed(KeyCode::S) {
        for (mut transform, _sprite) in query.iter_mut() {
            transform.translation.y += 0.5 * settings.cell_size;
        }
    }

    // D key moves cells right
    if keys.pressed(KeyCode::D) {
        for (mut transform, _sprite) in query.iter_mut() {
            transform.translation.x -= 0.5 * settings.cell_size;
        }
    }
    
    // A key moves cells left
    if keys.pressed(KeyCode::A) {
        for (mut transform, _sprite) in query.iter_mut() {
            transform.translation.x += 0.5 * settings.cell_size;
        }
    }
    
    // Constants for scaling
    const SCALE_MULTIPLIER: f32 = 1.02;
    const MAX_CELL_SIZE: f32 = 100.;
    const MIN_CELL_SIZE: f32 = 1.;
    
    // If Q is pressed, zooom in
    if keys.pressed(KeyCode::Q) {
        
        // Change cell_size by scale multiplier
        // If cells reach max size, do not scale anymore
        let mut changed: bool = true;
        settings.cell_size *= SCALE_MULTIPLIER;
        if settings.cell_size > MAX_CELL_SIZE {
            settings.cell_size = MAX_CELL_SIZE;
            changed = false;
        }

        // If cells are max size, do not scale
        if changed {
            for (mut transform, mut sprite) in query.iter_mut() {
                
                // First update scale then reposition cells
                sprite.custom_size = Some(Vec2::new(settings.cell_size, settings.cell_size));
                transform.translation.x *= SCALE_MULTIPLIER;
                transform.translation.y *= SCALE_MULTIPLIER;
            }
        }
    }
    
    // If E is pressed, zoom out
    if keys.pressed(KeyCode::E) {

        // Change cell_size by scale multiplier
        // If cells reach min size, do not scale anymore
        let mut changed: bool = true;
        settings.cell_size /= SCALE_MULTIPLIER;
        if settings.cell_size < MIN_CELL_SIZE {
            settings.cell_size = MIN_CELL_SIZE;
            changed = false;
        }
        
        // If cells are min size, do not scale
        if changed {
            for (mut transform, mut sprite) in query.iter_mut() {
                
                // First update scale then reposition cells
                sprite.custom_size = Some(Vec2::new(settings.cell_size, settings.cell_size));
                transform.translation.x /= SCALE_MULTIPLIER;
                transform.translation.y /= SCALE_MULTIPLIER;
            }
        }
    }
}