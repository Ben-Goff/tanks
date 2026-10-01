use bevy::{color::palettes::css::{PINK, RED, BLUE}, prelude::*};
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

#[derive(Component)]
pub struct Player {
}

#[derive(Component)]
pub struct Tank {
    pub turret_dir: Dir3,
}

#[derive(Component)]
pub struct Motion {
    pub pos: Vec3,
    pub vel: f32,
    pub dir: Dir3,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, setup))
        .add_systems(Update, (player_movement_input, sprite_movement, rotate_turret, shoot).chain())
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    commands.spawn((
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("tank.glb"))),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(1.0, 0.0, 0.0), Vec3::Z),
        Tank {turret_dir: Dir3::Y},
        Motion {pos: Vec3::ZERO, vel: 0.0, dir: Dir3::Y },
        Player{},
    ));
}

fn rotate_turret(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut turret_query: Query<(&Name, &mut Transform), Without<Player>>,
    player: Single<(&mut Tank, &Transform), With<Player>>,
) {
    let (mut player_tank, player_transform) = player.into_inner();
    if let Some(cursor_pos) = center_scale_cursor_position(camera, window) {
        for (name, mut transform) in &mut turret_query {
            if name.as_str() == "Turret_Main" {
                if let Ok(dir) = Dir3::new(cursor_pos - player_transform.translation) {
                    let local_dir_vec = player_transform.rotation.inverse() * dir;
                    player_tank.turret_dir = dir;
                    transform.look_to(local_dir_vec, Vec3::Y); 
                }
            }
        }
    }
}

// calculate center-origin cursor position, with y-axis inverted to match the 2D coordinate system
fn center_scale_cursor_position(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window, With<PrimaryWindow>>,
) -> Option<Vec3> {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position() {
        
        // 2. Project a 3D ray out of the camera matching the 2D cursor position
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
            
            // 3. Find where the ray intersects the ground's infinite flat plane
            if let Some(intersection_point) = ray.plane_intersection_point(
                Vec3::ZERO, InfinitePlane3d::new(Vec3::Z)
            ) {
                // intersection_point is a Vec3 containing your 3D world coordinates!
                return Some(intersection_point)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn player_movement_input(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    player: Single<&mut Motion, With<Player>>,
) {
    let mut player_motion = player.into_inner();

    if mouse_buttons.pressed(MouseButton::Left) {
        if let Some(cursor_pos) = center_scale_cursor_position(camera, window) {
            if let Ok(dir) = Dir3::new(cursor_pos - player_motion.pos) {
                player_motion.dir = dir;
                player_motion.vel = 5.0;
            }
        }
    } else {
        player_motion.vel = 0.0;
    }
}



fn sprite_movement(
    time: Res<Time>,
    mut shape_position: Query<(&mut Motion, &mut Transform)>
) {
    for (mut motion, mut transform) in &mut shape_position {
        let new_pos: Vec3 = motion.pos + motion.dir * motion.vel * time.delta_secs();
        motion.pos = new_pos;
        transform.translation = motion.pos;
        transform.look_to(motion.dir, Vec3::Z);
    }
}

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Motion, &mut Tank), With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let ( player_motion, player_tank) = player.into_inner();
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(PINK),
                ..default()
            })),
            Transform::from_translation(player_motion.pos).with_scale(Vec3::new(0.3, 1.0, 1.0)),
            Motion{pos: player_motion.pos, vel: 20.0, dir: player_tank.turret_dir},
        ));
    }
}