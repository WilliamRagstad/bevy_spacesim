use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_space_ship, spawn_camera, spawn_lignt))
        .add_systems(Update, move_space_ship)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_lignt(mut commands: Commands) {
    commands.spawn((
        PointLight {
            range: 10_000_000.,
            radius: 100_000.,
            intensity: 10_000_000.,
            ..default()
        },
        Transform::from_xyz(0., 0., 5.0),
    ));
}

#[derive(Component)]
struct SpaceShip {
    speed: f32,
    position: Vec2,
    rotation: f32,
}

fn spawn_space_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(10., 10.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("space_ship.png")),
            alpha_mode: AlphaMode::Blend,
            unlit: false,
            ..default()
        })),
        Transform::default(),
        SpaceShip {
            speed: 0.25,
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
