use bevy::prelude::*;

use crate::card::CardInfo;

#[derive(Component)]
pub struct Stack;

pub(super) fn stack(loc: Vec3) -> impl Bundle {
    (Transform::from_translation(loc), Pickable::default(), Stack)
}

pub(super) fn reorder_children(
    mut q_cards: Query<&mut Transform, With<CardInfo>>,
    q_stacks: Query<&Children, Changed<Children>>,
) {
    for children in q_stacks.iter() {
        let mut current_y = 0.;
        for child in children {
            let Ok(mut transform) = q_cards.get_mut(*child) else {
                continue;
            };
            transform.translation.x = 0.;
            transform.translation.y = current_y;
            current_y -= 10.0;
        }
    }
}

pub(super) fn drag_drop_combine(
    drag_drop: On<Pointer<DragDrop>>,
    mut q_cards: Query<(Entity, &Children), With<CardInfo>>,
    mut commands: Commands,
) {
    let orig_card_mesh_entity = drag_drop.dropped;
    let new_stack = drag_drop.entity;

    let (orig_card, _) = q_cards
        .iter_mut()
        .find(|(_, child)| child.contains(&orig_card_mesh_entity))
        .expect("Mesh being dragged should have card parent");

    println!("Changing transforms");
    commands
        .entity(orig_card)
        .remove_parent_in_place()
        .set_parent_in_place(new_stack);
}
