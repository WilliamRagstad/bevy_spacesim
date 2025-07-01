use crate::plugins::{
    camera::CameraPlugin, lighting::LightingPlugin, planet::PlanetPlugin,
    spaceship::SpaceshipPlugin, starfield::StarfieldPlugin,
};
use bevy::prelude::*;
use big_space::plugin::BigSpaceDefaultPlugins;

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())
        .add_plugins(BigSpaceDefaultPlugins)
        .add_plugins((
            SpaceshipPlugin,
            CameraPlugin,
            PlanetPlugin,
            LightingPlugin,
            StarfieldPlugin,
        ))
        .run();
}
