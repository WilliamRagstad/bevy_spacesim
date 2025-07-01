use bevy::prelude::*;
use big_space::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lignt);
    }
}

const SUN_BRIGHTNESS: f32 = 3.6 * 1e16;

fn spawn_lignt(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    big_space_query: Query<Entity, With<BigSpace>>,
) {
    let Ok(big_space_entity) = big_space_query.single() else {
        return; // No BigSpace found yet
    };

    let sun_entity = commands.spawn((
        Name::new("Sun"),
        PointLight {
            range: 10_000_000_000.,
            radius: 100_000_000.,
            intensity: SUN_BRIGHTNESS,
            ..default()
        },
        BigSpatialBundle {
            transform: Transform::from_xyz(700_000., 400_000., 500_000.),
            cell: GridCell::default(),
            ..default()
        },
    )).id();

    // Make sun a child of BigSpace
    commands.entity(big_space_entity).add_child(sun_entity);
}
