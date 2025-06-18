use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lignt);
    }
}

const SUN_BRIGHTNESS: f32 = 3.6 * 1e16;

fn spawn_lignt(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Sun"),
        // Mesh3d(meshes.add(Sphere::new(100_000.))),
        // MeshMaterial3d(materials.add(StandardMaterial {
        //     base_color: Color::WHITE,
        //     perceptual_roughness: 0.0,
        //     reflectance: 1.0,
        //     alpha_mode: AlphaMode::Blend,
        //     diffuse_transmission: 0.2,
        //     specular_transmission: 1.0,
        //     ..default()
        // })),
        PointLight {
            range: 10_000_000_000.,
            radius: 100_000_000.,
            intensity: SUN_BRIGHTNESS,
            ..default()
        },
        Transform::from_xyz(700_000., 400_000., 500_000.),
        Visibility::Visible,
        InheritedVisibility::default(),
    ));
}
