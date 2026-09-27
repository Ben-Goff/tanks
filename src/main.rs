use bevy::{color::palettes::css::{PINK, RED}, prelude::*};
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input, shoot, sprite_movement).chain())
        .run();
}

#[derive(Component)]
pub struct Player {
}

#[derive(Component)]
pub struct Motion {
    pub pos: Vec2,
    pub vel: Vec2,
    pub dir: Rot2,
}

pub struct KeyVelocity {
    pub vel: Vec2,
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    player: Single<&mut Motion, With<Player>>,
) {
    let key_movements = [
        (KeyCode::ArrowUp, Vec2::Y),
        (KeyCode::ArrowDown, -1.0 * Vec2::Y),
        (KeyCode::ArrowLeft, Vec2::X),
        (KeyCode::ArrowRight, -1.0 * Vec2::X),
    ];

    let mut total_vel = Vec2::ZERO;
    let mut total_dir = Vec2::ZERO;
    for (key, movement) in &key_movements {
        if keys.pressed(*key) {
            total_dir += movement;
        }
        if keys.just_pressed(*key) {
            total_vel += movement;
        } else if keys.just_released(*key) {
            total_vel -= movement;
        }
    }
    let normalized_dir = total_dir.normalize();
    let normalized_vel = total_vel.normalize();

    let mut player_info = player.into_inner();
    player_info.vel = normalized_vel;
    if normalized_dir != Rot2::IDENTITY {
        player_info.dir = normalized_dir;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(100.0))),
        MeshMaterial2d(materials.add(Color::from(PINK))),
        Transform::from_xyz(0.0,0.0,0.0),
        Motion{pos: Vec2::ZERO, vel: Vec2::ZERO, dir: Mat2::ZERO},
        Player{},
    ));
}

fn sprite_movement(
    time: Res<Time>,
    mut shape_position: Query<(&mut Motion)>
) {
    for (velocity, mut transform) in &mut shape_position {
        transform.translation.x += velocity.x_vel * 200.0 * time.delta_secs();
        transform.translation.y += velocity.y_vel * 200.0 * time.delta_secs();
    }
}

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Velocity, &mut Transform), With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let player_info = player.into_inner();
        let player_transform = player_info.1;
        let player_velocity = player_info.0;
        if player_velocity.x_vel != 0.0 || player_velocity.y_vel != 0.0 {
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(10.0))),
                MeshMaterial2d(materials.add(Color::from(RED))),
                Transform::from_xyz(player_transform.translation.x, player_transform.translation.y, 0.0),
                Velocity{x_vel: player_velocity.x_vel * 7.0, y_vel: player_velocity.y_vel * 7.0},
            ));
        }
    }
}