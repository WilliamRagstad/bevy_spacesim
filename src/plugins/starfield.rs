use bevy::prelude::*;

const SPACE_BACKGROUND_COLOR: Color = Color::srgb(0.05, 0.05, 0.1);

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(SPACE_BACKGROUND_COLOR));
    }
}
