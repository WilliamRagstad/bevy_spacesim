use crate::plugins::spaceship::Spaceship;
use bevy::{core_pipeline::motion_blur::MotionBlur, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_camera);
    }
}

const FOLLOW_DISTANCE: f32 = 25.0;
const FOLLOW_HEIGHT: f32 = 10.0;
const FOLLOW_Y_OFFSET: f32 = 5.0;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        MotionBlur {
            shutter_angle: 1.0,
            samples: 2,
        },
        // MSAA and Motion Blur together are not compatible on WebGL
        #[cfg(all(feature = "webgl2", target_arch = "wasm32", not(feature = "webgpu")))]
        Msaa::Off,
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
        let ship_pos = ship_transform.translation;
        let ship_forward = ship_transform.forward();
        let ship_up = ship_transform.up();
        cam_transform.translation =
            ship_pos - ship_forward * FOLLOW_DISTANCE + ship_up * FOLLOW_HEIGHT;
        let looking_at_pos = ship_pos + Vec3::Y * FOLLOW_Y_OFFSET;
        cam_transform.look_at(looking_at_pos, Vec3::Y);
    }
}
