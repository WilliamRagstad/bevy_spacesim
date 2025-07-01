use bevy::prelude::*;
use big_space::prelude::*;

const SPACE_BACKGROUND_COLOR: Color = Color::srgb(0.05, 0.05, 0.1);

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(SPACE_BACKGROUND_COLOR));
        app.add_systems(Startup, setup_skybox);
    }
}

fn setup_skybox(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    big_space_query: Query<Entity, With<BigSpace>>,
) {
    let Ok(big_space_entity) = big_space_query.single() else {
        return; // No BigSpace found yet
    };

    let skybox_texture = asset_server.load("space_bg.png");
    let skybox_material = materials.add(StandardMaterial {
        base_color_texture: Some(skybox_texture),
        perceptual_roughness: 0.0,
        reflectance: 0.0,
        cull_mode: Some(bevy::render::render_resource::Face::Front), // Cull front faces, show inside
        unlit: true,
        ..default()
    });

    let skybox_entity = commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere::new(1.)))),
        MeshMaterial3d(skybox_material),
        BigSpatialBundle {
            transform: Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(100_000_000_000.0)),
            cell: GridCell::default(),
            ..default()
        },
    )).id();

    // Make skybox a child of BigSpace
    commands.entity(big_space_entity).add_child(skybox_entity);
}
