use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct GamePlugin;

const SPEED: f32 = 500.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (pan_map, zoom_map));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
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
