use std::ops::Range;

use crate::ops::acos;
use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use noise::{NoiseFn, Perlin};
use rand::{Rng, RngCore};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planets);
    }
}

fn generate_planet_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    rng: &mut rand::rngs::ThreadRng,
) -> Handle<StandardMaterial> {
    let width = 128;
    let height = 128;

    let perlin = Perlin::new(rng.next_u32());
    let mut texture_data = Vec::with_capacity((width * height * 4) as usize);

    let hue1 = rng.random_range(0.0..360.0);
    let hue2 = (hue1 + rng.random_range(60.0..180.0)) % 360.0;
    let color1 = Color::hsl(hue1, 0.7, 0.4).to_linear();
    let color2 = Color::hsl(hue2, 0.7, 0.6).to_linear();

    for y in 0..height {
        for x in 0..width {
            let fx = x as f64 / width as f64;
            let fy = y as f64 / height as f64;
            let noise_val = perlin.get([fx * 8.0, fy * 8.0]);

            // Normalize to [0,1]
            let t = ((noise_val + 1.0) * 0.5) as f32;
            let r = color1.red * (1.0 - t) + color2.red * t;
            let g = color1.green * (1.0 - t) + color2.green * t;
            let b = color1.blue * (1.0 - t) + color2.blue * t;

            texture_data.push((r * 255.0) as u8);
            texture_data.push((g * 255.0) as u8);
            texture_data.push((b * 255.0) as u8);
            texture_data.push(255);
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::linear();
    let image_handle = images.add(image);
    let roughness = rng.random_range(0.1..0.9);
    let metallic = rng.random_range(0.0..0.6);
    materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        base_color: Color::WHITE,
        perceptual_roughness: roughness,
        metallic,
        ..default()
    })
}

const PLANET_COUNT: usize = 30;
const PLANET_DISTANCE: Range<f32> = 600_000.0..1_000_000.0;
const PLANET_SCALE: Range<f32> = 5_000.0..30_000.0;

fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let planet_mesh = meshes.add(Mesh::from(Sphere { radius: 1.0 }));
    let mut rng = rand::rng();
    for _ in 0..PLANET_COUNT {
        let dist = rng.random_range(PLANET_DISTANCE);
        let theta = rng.random_range(0.0..(2.0 * std::f32::consts::PI));
        let phi = acos(rng.random_range(-1.0..1.0));
        let x = dist * phi.sin() * theta.cos();
        let y = dist * phi.cos();
        let z = dist * phi.sin() * theta.sin();
        let position = Vec3::new(x, y, z);
        let planet_material = generate_planet_material(&mut materials, &mut images, &mut rng);
        let scale = rng.random_range(PLANET_SCALE);
        commands.spawn((
            Mesh3d(planet_mesh.clone()),
            MeshMaterial3d(planet_material.clone()),
            Transform::from_translation(position).with_scale(Vec3::splat(scale)),
            Visibility::Visible,
            InheritedVisibility::default(),
        ));
    }
}
