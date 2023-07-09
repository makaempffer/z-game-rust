//! Renders a 2D scene containing a single, moving sprite.

use bevy::{prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    Idle
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("../assets/player_hk_stand.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
    Player {};
}
#[derive(Component, Debug)]
struct Player {  }

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>, 
    mut sprite_position: Query<(&mut Direction, 
        &mut Transform)>, 
        keyboard_input: Res<Input<KeyCode>>
) {
    let k_inputs: Vec<KeyCode> = vec![KeyCode::A, KeyCode::D, KeyCode::S, KeyCode::W];
    for (mut logo, mut transform) in &mut sprite_position {
        let speed: f32 = 150.;
        match *logo {
            Direction::Up => transform.translation.y += speed * time.delta_seconds(),
            Direction::Down => transform.translation.y -= speed * time.delta_seconds(),
            Direction::Right => transform.translation.x += speed * time.delta_seconds(),
            Direction::Left => transform.translation.x -= speed * time.delta_seconds(),
            Direction::Idle => {
                transform.translation.x += 0.;
                transform.translation.y += 0.;
            }
        }
        if keyboard_input.any_just_pressed(k_inputs.clone()) || keyboard_input.any_pressed(k_inputs.clone()) {
            if keyboard_input.just_pressed(KeyCode::W) {
                *logo = Direction::Up;
            }
        
            if keyboard_input.just_pressed(KeyCode::S) {
                *logo = Direction::Down;
            }
        
            if keyboard_input.just_pressed(KeyCode::D) {
                *logo = Direction::Right;
            } 
    
            if keyboard_input.just_pressed(KeyCode::A) {
                *logo = Direction::Left;
            } 
        } else {
            *logo = Direction::Idle;
        }

        


    }
}
