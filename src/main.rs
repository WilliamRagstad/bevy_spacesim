use crate::plugins::{
    camera::CameraPlugin, lighting::LightingPlugin, planet::PlanetPlugin,
    spaceship::SpaceshipPlugin, starfield::StarfieldPlugin,
};
use bevy::prelude::*;

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            SpaceshipPlugin,
            CameraPlugin,
            PlanetPlugin,
            LightingPlugin,
            StarfieldPlugin,
        ))
        .run();
}
