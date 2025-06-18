use crate::plugins::spaceship::Spaceship;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_camera);
    }
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
