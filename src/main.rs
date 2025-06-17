use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_space_ship, spawn_camera))
        .add_systems(Update, move_space_ship)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(0.0, 0.0, 100.0),
            ..default()
        },
        GlobalTransform::default(),
    ));
}

#[derive(Component)]
struct SpaceShip {
    speed: f32,
    position: Vec2,
    rotation: f32,
}

fn spawn_space_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("space_ship.png");
    commands.spawn((
        Sprite {
            image: texture_handle,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(0.5),
            ..default()
        },
        GlobalTransform::default(),
        SpaceShip {
            speed: 2.0,
            position: Vec2::ZERO,
            rotation: 0.0,
        },
    ));
}

fn move_space_ship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut SpaceShip)>,
) {
    for (mut transform, mut spaceship) in query.iter_mut() {
        let mut direction = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if direction != Vec2::ZERO {
            spaceship.position = direction.normalize();
            transform.translation.x += spaceship.position.x * spaceship.speed;
            transform.translation.y += spaceship.position.y * spaceship.speed;
            spaceship.rotation = -direction.angle_to(Vec2::X) - std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(spaceship.rotation);
        }
    }
}
