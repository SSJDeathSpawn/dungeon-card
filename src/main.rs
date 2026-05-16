use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::card::CardPlugin;
use crate::debug::DebugPlugin;
use crate::game::GamePlugin;
use crate::map::MapPlugin;
mod card;
pub mod constant;
mod debug;
mod game;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(MeshPickingPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(MapPlugin)
        .add_plugins(CardPlugin)
        .add_plugins(DebugPlugin)
        .insert_resource::<MeshPickingSettings>(MeshPickingSettings {
            require_markers: true,
            ..default()
        })
        .insert_resource::<ClearColor>(ClearColor(Color::BLACK))
        .run();
}
