use bevy::prelude::*;

use crate::{
    card::{CardInfo, card_visual, visual::CardVisualInfo},
    game::{MouseCoords, YSort},
};

#[derive(Component)]
pub struct FollowCard;

pub(super) fn follow_card(card_visuals: &CardVisualInfo, card_info: &CardInfo) -> impl Bundle {
    (
        Transform::default(),
        Pickable::IGNORE,
        YSort { z: 1.0 },
        FollowCard,
        card_visual(card_visuals, card_info.card_type, Pickable::IGNORE),
    )
}

pub(super) fn create_follow_card(
    event: On<Pointer<DragStart>>,
    mut q_cards: Query<(&CardInfo, &mut Visibility)>,
    card_visuals: Res<CardVisualInfo>,
    mut commands: Commands,
) {
    let card_mesh = event.entity;
    let (card, mut visibility) = q_cards
        .get_mut(card_mesh)
        .expect("Every card should have CardInfo");
    *visibility = Visibility::Hidden;
    commands.spawn(follow_card(&card_visuals, card));
}

pub(super) fn delete_follow_card(
    event: On<Pointer<DragEnd>>,
    mut q_cards: Query<&mut Visibility>,
    ghost_card: Single<Entity, With<FollowCard>>,
    mut commands: Commands,
) {
    let card_mesh = event.entity;
    let mut visibility = q_cards
        .get_mut(card_mesh)
        .expect("Every card should have CardInfo");
    *visibility = Visibility::Visible;
    commands.entity(*ghost_card).despawn_children().despawn();
}

pub(super) fn follow_card_follow_mouse(
    mouse_coord: Res<MouseCoords>,
    mut q_ghost_cards: Query<&mut Transform, With<FollowCard>>,
) {
    let Ok(mut transform) = q_ghost_cards.single_mut() else {
        return;
    };

    transform.translation.x = mouse_coord.0.x;
    transform.translation.y = mouse_coord.0.y;
}
