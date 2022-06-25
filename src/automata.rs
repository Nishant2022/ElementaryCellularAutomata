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
            .add_system(mouse_button_input);
    }
}

fn cell_spawn_system(mut commands: Commands, win_size: Res<WinSize>) {
    let num_cells: usize = 257;
    let cell_size: f32 = win_size.w / num_cells as f32 * 2.0;
    
    let init_rule: u8 = 30;

    commands.insert_resource(CellSettings {
        cell_size: cell_size,
        num_cells: num_cells as u32,
        dead_color: Color::BLACK,
        alive_color: Color::WHITE,
        rule_num: init_rule,
        rule: get_rule(init_rule),
    });

    let mut cell_grid = CellGrid {grid: Vec::new()};

    for i in 0..num_cells {
        cell_grid.grid.push(Vec::new());
        for j in 0..num_cells {
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

fn update_cell_grid_system(mut cell_grid: ResMut<CellGrid>, cell_settings: Res<CellSettings>) {
    let num_cells = cell_settings.num_cells;
    
    for i in 0..num_cells {
        cell_grid.grid[i as usize][0].state = CellState::Dead;
    }

    cell_grid.grid[(num_cells / 2) as usize][0].state = CellState::Alive;

    for i in 0..num_cells as usize {
        for j in 1..num_cells as usize {

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

            if cell_settings.rule[counter as usize] { cell_grid.grid[i][j].state = CellState::Alive}
            else { cell_grid.grid[i][j].state = CellState::Dead }

        }
    }
}

fn color_grid_system(mut query: Query<(&mut Sprite, &mut Cell)>, cell_grid: Res<CellGrid>, cell_settings: Res<CellSettings>) {
    for (mut sprite, mut cell) in query.iter_mut() {
        cell.state = cell_grid.grid[cell.position_x as usize][cell.position_y as usize].state;
        match cell.state {
            CellState::Alive => sprite.color = cell_settings.alive_color,
            CellState::Dead => sprite.color = cell_settings.dead_color,
        }
    }
}

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

fn mouse_button_input (buttons: Res<Input<MouseButton>>, 
    mut cell_settings: ResMut<CellSettings>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        cell_settings.rule_num -= 1;
        cell_settings.rule = get_rule(cell_settings.rule_num);
        println!("{}", cell_settings.rule_num);
    }
    
    if buttons.just_pressed(MouseButton::Left) {
        cell_settings.rule_num += 1;
        cell_settings.rule = get_rule(cell_settings.rule_num);
        println!("{}", cell_settings.rule_num);
    }
}