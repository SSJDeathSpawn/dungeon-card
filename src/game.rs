use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct GamePlugin;

#[derive(Component)]
pub struct YSort {
    pub z: f32,
}

#[derive(Resource, Default)]
pub struct MouseCoords(pub Vec2);

const SPEED: f32 = 500.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseCoords>()
            .add_systems(Startup, setup_camera)
            .add_systems(Startup, disable_window_picking)
            .add_systems(Update, (pan_map, zoom_map))
            .add_systems(Update, cursor_system)
            .add_systems(Update, y_sort);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MeshPickingCamera));
}

fn disable_window_picking(mut commands: Commands, window_query: Query<Entity, With<Window>>) {
    for window_entity in window_query.iter() {
        commands.entity(window_entity).insert(Pickable::IGNORE);
    }
}

fn pan_map(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = q_camera.single_mut() else {
        return;
    };

    let mut delta = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        delta.y += 1.0
    }

    if keys.pressed(KeyCode::KeyA) {
        delta.x -= 1.0
    }

    if keys.pressed(KeyCode::KeyS) {
        delta.y -= 1.0
    }

    if keys.pressed(KeyCode::KeyD) {
        delta.x += 1.0
    }

    transform.translation += delta * SPEED * time.delta_secs();
}

fn zoom_map(mut q_camera: Query<&mut Projection>, mut scroll_evr: MessageReader<MouseWheel>) {
    let Ok(mut projection) = q_camera.single_mut() else {
        return;
    };

    let Projection::Orthographic(ref mut ortho) = *projection else {
        return;
    };

    for ev in scroll_evr.read() {
        ortho.scale *= 1.0 - ev.y * 0.01;
        ortho.scale = ortho.scale.clamp(0.1, 10.0);
    }
}

fn y_sort(mut q: Query<(&mut Transform, &YSort)>) {
    for (mut tf, ysort) in q.iter_mut() {
        tf.translation.z = ysort.z - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * tf.translation.y))));
    }
}

fn cursor_system(
    mut mycoords: ResMut<MouseCoords>,
    window: Single<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single().expect("Surely camera exists");

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}
