use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_space_ship)
            .add_systems(Update, move_space_ship);
    }
}

const THRUST_ACCEL: f32 = 1_000.0;
const BOOST_ACCEL: f32 = 40_000.0; // Boost acceleration
const DRAG_FRICTION: f32 = 0.99; // 1.0 = no drag, lower = more drag
const ROLL_SPEED: f32 = 1.5;
const MOUSE_SENSITIVITY: f32 = 0.001;
const MIN_MOUSE_OFFSET: f32 = 30.0;

#[derive(Component)]
pub struct Spaceship {
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
            if keyboard_input.pressed(KeyCode::ShiftLeft) {
                // Boost the spaceship
                spaceship.velocity += forward * BOOST_ACCEL * dt;
            } else {
                // Normal thrust
                spaceship.velocity += forward * THRUST_ACCEL * dt;
            }
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
        // use cursor position relative to window center
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
