use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lignt);
    }
}

fn spawn_lignt(mut commands: Commands) {
    commands.spawn((
        PointLight {
            range: 100_000_000.,
            radius: 100_000_000.,
            intensity: 10_000_000.,
            ..default()
        },
        Transform::from_xyz(0., 0., 5.0),
    ));
}
