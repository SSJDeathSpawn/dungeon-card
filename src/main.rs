use bevy::prelude::*;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource::<ClearColor>(ClearColor(Color::BLACK))
        .run();
}
