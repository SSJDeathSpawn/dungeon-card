use bevy::{platform::collections::HashSet, prelude::*};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.init_resource::<Seen>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, detect_new);
    }
}

#[derive(Resource, Default)]
struct Seen(HashSet<Entity>);

fn detect_new(mut seen: ResMut<Seen>, mut commands: Commands, entities: Query<Entity>) {
    for e in &entities {
        if seen.0.insert(e) {
            commands.entity(e).log_components();
        }
    }
}
