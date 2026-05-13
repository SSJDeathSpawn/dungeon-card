use bevy::prelude::*;

use crate::game::YSort;
use crate::{constant, map::BoardSettings};

pub struct CardPlugin;

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct Stack;

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
        Pickable::default(),
        YSort { z: 1.0 },
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
                Pickable::IGNORE,
            )
        ],
    )
}

fn stack(
    loc: Vec3,
    card_mesh: Handle<Mesh>,
    card_color: Handle<ColorMaterial>,
    card_outline: Handle<Mesh>,
    outline_color: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Transform::from_translation(loc),
        Pickable::default(),
        Stack,
        children![card(
            Vec3::new(0., 0., 0.),
            card_mesh,
            card_color,
            card_outline,
            outline_color
        )],
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
    let card_color1 = materials.add(Color::linear_rgb(1., 1., 1.));
    let card_color2 = materials.add(Color::linear_rgb(1., 0., 0.));

    let card_outline = meshes.add(
        Rectangle::new(
            constant::CARD::FACTOR,
            constant::CARD::FACTOR / constant::CARD::RATIO,
        )
        .to_ring(constant::CARD::THICKNESS),
    );
    let outline_color = materials.add(Color::linear_rgb(0., 0., 0.));

    commands
        .spawn(stack(
            Vec3::new(-20., 0., 0.),
            card_mesh.clone(),
            card_color1.clone(),
            card_outline.clone(),
            outline_color.clone(),
        ))
        .observe(move_stack_with_mouse)
        .observe(drop_stack_combine);

    commands
        .spawn(stack(
            Vec3::new(20., 0., 0.),
            card_mesh,
            card_color2,
            card_outline,
            outline_color,
        ))
        .observe(move_stack_with_mouse)
        .observe(drop_stack_combine);
}

fn move_stack_with_mouse(
    drag: On<Pointer<Drag>>,
    q_camera: Query<&Projection>,
    q_map: Query<&BoardSettings>,
    mut q_stacks: Query<&mut Transform, With<Stack>>,
    mut commands: Commands,
) {
    commands.entity(drag.entity).log_components();
    let mut transform = q_stacks.get_mut(drag.entity).unwrap();
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

fn drop_stack_combine(
    drag_drop: On<Pointer<DragDrop>>,
    mut commands: Commands,
    q_stacks: Query<&Children, With<Stack>>,
    mut q_cards: Query<&mut Transform, With<Card>>,
) {
    println!("Dropped entity: {:?}", drag_drop.dropped);
    println!("Target entity: {:?}", drag_drop.event_target());

    let current_stack_entity = drag_drop.dropped;
    let new_stack_entity = drag_drop.event_target();

    //Get cards in old stack
    let Ok(orig_cards) = q_stacks.get(current_stack_entity) else {
        println!(
            "Failed to get children for dropped stack entity: {:?}",
            current_stack_entity
        );
        return;
    };

    //Get cards in new stack
    let Ok(new_cards) = q_stacks.get(new_stack_entity) else {
        println!(
            "Failed to get children for target stack entity: {:?}",
            new_stack_entity
        );
        return;
    };

    //Get bottomost card in new stack
    let last_card = new_cards.last().unwrap();

    //Get bottomost card's y
    let mut transform_y = {
        let Ok(last_transform) = q_cards.get(*last_card) else {
            return;
        };
        last_transform.translation.y
    };

    for child in orig_cards.iter() {
        transform_y += 5.0;
        let mut card_entity_commands = commands.entity(child);
        card_entity_commands.set_parent_in_place(new_stack_entity);
        let Ok(mut card_transform) = q_cards.get_mut(child) else {
            return;
        };
        card_transform.translation = Vec3::new(0., transform_y, 0.);
    }

    commands.entity(current_stack_entity).despawn();
}
