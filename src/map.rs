use bevy::prelude::*;
pub struct MapPlugin;

#[derive(Component)]
pub struct MapSettings {
    pub size: Vec2,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}
