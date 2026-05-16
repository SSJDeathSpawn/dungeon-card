use bevy::prelude::*;
use enum_map::EnumMap;

use crate::card::CardType;

#[derive(Clone)]
pub struct CardTheme(pub EnumMap<CardType, Handle<ColorMaterial>>);

#[derive(Component)]
pub struct CardFront;

#[derive(Component)]
pub struct CardOutline;

#[derive(Resource, Clone)]
pub struct CardVisualInfo {
    pub card_front_mesh: Handle<Mesh>,
    pub card_outline_mesh: Handle<Mesh>,
    pub card_outline_color: Handle<ColorMaterial>,
    pub card_theme: CardTheme,
}

pub(super) fn card_visual(
    card_visuals: &CardVisualInfo,
    card_type: CardType,
    pickable: Pickable,
) -> impl Bundle {
    let CardVisualInfo {
        card_front_mesh: card_mesh,
        card_outline_mesh: outline_mesh,
        card_outline_color: outline_color,
        card_theme,
    } = card_visuals;

    children![
        (
            Visibility::Inherited,
            CardFront,
            Mesh2d(card_mesh.clone()),
            MeshMaterial2d(card_theme.0[card_type].clone()),
            pickable
        ),
        (
            Visibility::Inherited,
            CardOutline,
            Mesh2d(outline_mesh.clone()),
            MeshMaterial2d(outline_color.clone()),
            Pickable::IGNORE,
        ),
    ]
}
