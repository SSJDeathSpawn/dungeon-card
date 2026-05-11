use bevy::prelude::*;

use crate::{constant, map::BoardSettings};

pub struct CardPlugin;

#[derive(Component)]
pub struct Card;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_card);
    }
}

fn card(
    loc: Vec3,
    card_mesh: Handle<Mesh>,
    card_color: Handle<ColorMaterial>,
    card_outline: Handle<Mesh>,
    outline_color: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Transform::from_translation(loc),
        Card,
        children![
            (
                Mesh2d(card_mesh),
                MeshMaterial2d(card_color),
                Pickable::default(),
            ),
            (
                Mesh2d(card_outline),
                MeshMaterial2d(outline_color),
                Pickable::default(),
            )
        ],
    )
}

fn spawn_card(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let card_mesh = meshes.add(Rectangle::new(
        constant::CARD::FACTOR,
        constant::CARD::FACTOR / constant::CARD::RATIO,
    ));
    let card_color = materials.add(Color::linear_rgb(1., 1., 1.));

    let card_outline = meshes.add(
        Rectangle::new(
            constant::CARD::FACTOR,
            constant::CARD::FACTOR / constant::CARD::RATIO,
        )
        .to_ring(constant::CARD::THICKNESS),
    );
    let outline_color = materials.add(Color::linear_rgb(0., 0., 0.));

    commands
        .spawn(card(
            Vec3::new(-20., 0., 0.),
            card_mesh.clone(),
            card_color.clone(),
            card_outline.clone(),
            outline_color.clone(),
        ))
        .observe(move_card_with_mouse);

    commands
        .spawn(card(
            Vec3::new(20., 0., 0.),
            card_mesh,
            card_color,
            card_outline,
            outline_color,
        ))
        .observe(move_card_with_mouse);
}

fn move_card_with_mouse(
    drag: On<Pointer<Drag>>,
    q_camera: Query<&Projection>,
    q_map: Query<&BoardSettings>,
    mut q_cards: Query<&mut Transform>,
) {
    let mut transform = q_cards.get_mut(drag.entity).unwrap();
    let Ok(camera) = q_camera.single() else {
        return;
    };
    let Projection::Orthographic(ortho) = camera else {
        return;
    };
    let Ok(board_setting) = q_map.single() else {
        return;
    };

    transform.translation.x = (transform.translation.x + drag.delta.x * ortho.scale).clamp(
        (-board_setting.size.x + constant::CARD::FACTOR) / 2. + constant::MAP::THICKNESS,
        (board_setting.size.x - constant::CARD::FACTOR) / 2. - constant::MAP::THICKNESS,
    );
    transform.translation.y = (transform.translation.y - drag.delta.y * ortho.scale).clamp(
        (-board_setting.size.y + constant::CARD::FACTOR / constant::CARD::RATIO) / 2.
            + constant::MAP::THICKNESS,
        (board_setting.size.y - constant::CARD::FACTOR / constant::CARD::RATIO) / 2.
            - constant::MAP::THICKNESS,
    );
}
