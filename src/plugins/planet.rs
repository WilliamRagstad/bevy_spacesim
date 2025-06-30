use std::ops::Range;

use crate::ops::acos;
use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    math::ops::abs,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use big_space::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::{Rng, RngCore};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planets);
    }
}

const PLANET_MATERIAL_SIZE: u32 = 512; // Size of the planet texture
const PLANET_MATERIAL_CONTRAST: f32 = 7.5; // Higher = more contrast
const PLANET_MATERIAL_COLORS: Range<usize> = 2..4; // Number of colors to blend

fn generate_planet_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    rng: &mut rand::rngs::ThreadRng,
) -> Handle<StandardMaterial> {
    let perlin = Perlin::new(rng.next_u32());
    let mut texture_data =
        Vec::with_capacity((PLANET_MATERIAL_SIZE * PLANET_MATERIAL_SIZE * 4) as usize);
    let mut rng = rand::rng();
    let planet_material_colors = rng.random_range(PLANET_MATERIAL_COLORS);

    let mut hues = Vec::with_capacity(planet_material_colors);
    let mut prev_hue = 0.0;
    for _ in 0..planet_material_colors {
        // Generate a random hue, ensuring it is not too close to the previous one
        let mut hue = rng.random_range(0.0..360.0);
        while abs(hue - prev_hue) < 60.0 {
            hue = rng.random_range(0.0..360.0);
        }
        hues.push(hue);
        prev_hue = hue;
    }
    let colors = hues
        .into_iter()
        .map(|hue| {
            // Convert hue to linear RGB color
            Color::hsl(hue, 0.7, 0.5).to_linear()
        })
        .collect::<Vec<_>>();

    // Increase contrast by sharpening the blend factor with a power curve
    for y in 0..PLANET_MATERIAL_SIZE {
        for x in 0..PLANET_MATERIAL_SIZE {
            let fx = x as f64 / PLANET_MATERIAL_SIZE as f64;
            let fy = y as f64 / PLANET_MATERIAL_SIZE as f64;
            let noise_val = perlin.get([fx * 8.0, fy * 8.0]);

            // Normalize to [0,1]
            let mut t = ((noise_val + 1.0) * 0.5) as f32;
            // Sharpen the transition
            t = t.powf(PLANET_MATERIAL_CONTRAST)
                / (t.powf(PLANET_MATERIAL_CONTRAST) + (1.0 - t).powf(PLANET_MATERIAL_CONTRAST));

            let n = planet_material_colors;
            let scaled = t * (n as f32 - 1.0);
            let idx = scaled.floor() as usize;
            let frac = scaled - idx as f32;

            let color1 = &colors[idx.min(n - 1)];
            let color2 = &colors[(idx + 1).min(n - 1)];

            let r = color1.red * (1.0 - frac) + color2.red * frac;
            let g = color1.green * (1.0 - frac) + color2.green * frac;
            let b = color1.blue * (1.0 - frac) + color2.blue * frac;

            texture_data.push((r * 255.0) as u8);
            texture_data.push((g * 255.0) as u8);
            texture_data.push((b * 255.0) as u8);
            texture_data.push(255);
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width: PLANET_MATERIAL_SIZE,
            height: PLANET_MATERIAL_SIZE,
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
const PLANET_DISTANCE: Range<f32> = 200_000.0..1_000_000.0;
const PLANET_SCALE: Range<f32> = 5_000.0..30_000.0;

fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    big_space_query: Query<Entity, With<BigSpace>>,
) {
    let Ok(big_space_entity) = big_space_query.single() else {
        return; // No BigSpace found yet
    };

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
        
        let planet_entity = commands.spawn((
            Mesh3d(planet_mesh.clone()),
            MeshMaterial3d(planet_material.clone()),
            BigSpatialBundle {
                transform: Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                cell: GridCell::default(),
                ..default()
            },
        )).id();

        // Make planet a child of BigSpace
        commands.entity(big_space_entity).add_child(planet_entity);
    }
}
