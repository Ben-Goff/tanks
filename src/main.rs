use bevy::{color::palettes::css::{PINK, RED, BLUE}, prelude::*};
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

#[derive(Component)]
pub struct Player {
}

#[derive(Component)]
pub struct Motion {
    pub pos: Vec2,
    pub vel: f32,
    pub dir: Dir2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement_input, shoot, sprite_movement).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(Color::from(PINK))),
        Transform::from_xyz(0.0,0.0,0.0),
        Motion{pos: Vec2::ZERO, vel: 0.0, dir: Dir2::EAST},
        Player{},
    ));
}

// calculate center-origin cursor position, with y-axis inverted to match the 2D coordinate system
fn center_scale_cursor_position(
    window: Single<&Window, With<PrimaryWindow>>,
) -> Option<Vec2> {
    window.cursor_position().map(|cursor_pos| {
        let center: Vec2 = window.resolution.size() / 2.0;
        (cursor_pos - center) * Vec2::new(1.0, -1.0)
    })
}

fn player_movement_input(
    window: Single<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    player: Single<&mut Motion, With<Player>>,
) {
    let mut player_info = player.into_inner();

    if mouse_buttons.pressed(MouseButton::Left) {
        if let Some(cursor_pos) = center_scale_cursor_position(window) {
            if let Ok(dir) = Dir2::new(cursor_pos - player_info.pos) {
                player_info.dir = dir;
                player_info.vel = 100.0;
            }
        }
    } else {
        player_info.vel = 0.0;
    }
}

fn sprite_movement(
    time: Res<Time>,
    mut shape_position: Query<(&mut Motion, &mut Transform)>
) {
    for (mut motion, mut transform) in &mut shape_position {
        let new_pos: Vec2 = motion.pos + motion.dir * motion.vel * time.delta_secs();
        motion.pos = new_pos;
        transform.translation = new_pos.extend(0.0);
    }
}

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    player: Single<(&Motion, &Transform), With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let (player_motion, player_transform) = player.into_inner();
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(BLUE))),
            Transform::from_xyz(player_transform.translation.x, player_transform.translation.y, 0.0),
            Motion{pos: player_motion.pos, vel: 500.0, dir: player_motion.dir},
        ));
    }
}