use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::Rng;

use crate::ops::acos;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_space_ship, spawn_camera, spawn_planets, spawn_lignt),
        )
        .add_systems(Update, (move_space_ship, follow_camera))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

fn follow_camera(
    mut query_cam: Query<&mut Transform, (With<Camera3d>, Without<Spaceship>)>,
    query_ship: Query<&Transform, With<Spaceship>>,
) {
    if let (Ok(mut cam_transform), Ok(ship_transform)) =
        (query_cam.single_mut(), query_ship.single())
    {
        let follow_distance = 25.0;
        let follow_height = 10.0;
        let ship_pos = ship_transform.translation;
        let ship_forward = ship_transform.forward();
        let ship_up = ship_transform.up();
        cam_transform.translation =
            ship_pos - ship_forward * follow_distance + ship_up * follow_height;
        cam_transform.look_at(ship_pos, Vec3::Y);
    }
}

const PLANET_COUNT: usize = 30;

fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planet_mesh = meshes.add(Mesh::from(Sphere { radius: 1.0 }));
    let mut rng = rand::rng();
    for _ in 0..PLANET_COUNT {
        let dist = rng.random_range(60.0..120.0);
        let theta = rng.random_range(0.0..(2.0 * std::f32::consts::PI));
        let phi = acos(rng.random_range(-1.0..1.0));
        let x = dist * phi.sin() * theta.cos();
        let y = dist * phi.cos();
        let z = dist * phi.sin() * theta.sin();
        let position = Vec3::new(x, y, z);
        let color = Color::hsl(rng.random_range(0.0..360.0), 0.8, 0.5);
        let emissive_strength = rng.random_range(0.0..1.0);
        let planet_material = materials.add(StandardMaterial {
            base_color: color,
            emissive: color.with_luminance(emissive_strength).into(),
            ..Default::default()
        });
        let scale = rng.random_range(0.5..3.0);
        commands.spawn((
            Mesh3d(planet_mesh.clone()),
            MeshMaterial3d(planet_material.clone()),
            Transform::from_translation(position).with_scale(Vec3::splat(scale)),
        ));
    }
}

fn spawn_lignt(mut commands: Commands) {
    commands.spawn((
        PointLight {
            range: 100_000_000.,
            radius: 1_000_000.,
            intensity: 10_000_000.,
            ..default()
        },
        Transform::from_xyz(0., 0., 5.0),
    ));
}

const THRUST_ACCEL: f32 = 15.0;
const DRAG_FRICTION: f32 = 0.99; // 1.0 = no drag, lower = more drag
const ROLL_SPEED: f32 = 1.5;
const MOUSE_SENSITIVITY: f32 = 0.001;
const MIN_MOUSE_OFFSET: f32 = 10.0;

#[derive(Component)]
struct Spaceship {
    velocity: Vec3,
}

fn spawn_space_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a local scene with the 3D model and the rotation correction
    let model_scene = (
        SceneRoot(asset_server.load("space_fighter.glb#Scene0")),
        Transform::from_rotation(Quat::from_rotation_y(FRAC_PI_2)),
    );

    // Spawn the spaceship entity with the Spaceship component and a transform,
    // and add the model scene as a child
    commands
        .spawn((
            Transform::from_rotation(Quat::from_rotation_z(0.0)).looking_to(-Dir3::Z, Dir3::Y),
            Spaceship {
                velocity: Vec3::ZERO,
            },
            Visibility::Visible,
            InheritedVisibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn(model_scene);
        });
}

fn move_space_ship(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // mut mouse_input: EventReader<MouseMotion>,
    windows: Query<&Window>,
    mut query: Query<(&mut Transform, &mut Spaceship)>,
) {
    let window = if let Ok(w) = windows.single() {
        w
    } else {
        return;
    };
    let window_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    for (mut transform, mut spaceship) in query.iter_mut() {
        let dt = time.delta_secs();
        if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            // Accelerate the spaceship
            let forward = transform.forward();
            spaceship.velocity += forward * THRUST_ACCEL * dt;
        }
        if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            // Decelerate the spaceship
            let forward = transform.forward();
            spaceship.velocity -= forward * THRUST_ACCEL * dt;
        }
        // Update the spaceship's position based on its velocity
        transform.translation += spaceship.velocity * dt;

        // Apply drag to gradually slow down the ship
        spaceship.velocity *= DRAG_FRICTION;

        if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            // Roll the spaceship left
            transform.rotate_local_z(ROLL_SPEED * dt);
        }
        if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            // Roll the spaceship right
            transform.rotate_local_z(-ROLL_SPEED * dt);
        }
        // Mouse look controls (yaw & pitch)
        // let mut yaw_delta = 0.0;
        // let mut pitch_delta = 0.0;
        // for motion in mouse_input.read() {
        //     yaw_delta += motion.delta.x * MOUSE_SENSITIVITY;
        //     pitch_delta += -motion.delta.y * MOUSE_SENSITIVITY;
        // }
        // if yaw_delta != 0.0 {
        //     transform.rotate_local_y(-yaw_delta);
        // }
        // if pitch_delta != 0.0 {
        //     transform.rotate_local_x(pitch_delta);
        // }
        // Mouse look: use cursor position relative to window center
        if let Some(cursor_pos) = window.cursor_position() {
            let offset = cursor_pos - window_center;
            if offset.length() < MIN_MOUSE_OFFSET {
                continue; // Ignore small movements
            }
            let yaw_delta = -offset.x * MOUSE_SENSITIVITY * dt;
            let pitch_delta = -offset.y * MOUSE_SENSITIVITY * dt;

            if yaw_delta.abs() > 0.0001 {
                transform.rotate_local_y(yaw_delta);
            }
            if pitch_delta.abs() > 0.0001 {
                transform.rotate_local_x(pitch_delta);
            }
        }
    }
}
