use bevy::prelude::*;
use enum_map::{Enum, enum_map};

use crate::card::follow::{create_follow_card, delete_follow_card, follow_card_follow_mouse};
use crate::card::stack::{drag_drop_combine, reorder_children, stack};
use crate::card::visual::{CardTheme, CardVisualInfo, card_visual};
use crate::constant;
use crate::game::YSort;

pub struct CardPlugin;

mod follow;
mod stack;
mod visual;

#[derive(Enum, Clone, Copy)]
pub enum CardType {
    Demon,
    Normal,
    Imp,
}

impl From<CardType> for CardInfo {
    fn from(value: CardType) -> Self {
        Self { card_type: value }
    }
}

#[derive(Component, Clone)]
pub struct CardInfo {
    pub card_type: CardType,
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, follow_card_follow_mouse)
            .add_systems(Update, reorder_children);
    }
}

fn card(loc: Vec3, card_visuals: &CardVisualInfo, card_info: &CardInfo) -> impl Bundle {
    (
        Transform::from_translation(loc),
        Pickable::default(),
        YSort { z: 1.0 },
        Visibility::Visible,
        card_info.clone(),
        card_visual(card_visuals, card_info.card_type, Pickable::default()),
    )
}

fn startup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let card_visuals = setup(&mut meshes, &mut materials, &mut commands);
    spawn_card(&mut commands, card_visuals);
}

fn setup(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) -> CardVisualInfo {
    let card_mesh = meshes.add(Rectangle::new(
        constant::CARD::FACTOR,
        constant::CARD::FACTOR / constant::CARD::RATIO,
    ));
    let card_outline = meshes.add(
        Rectangle::new(
            constant::CARD::FACTOR,
            constant::CARD::FACTOR / constant::CARD::RATIO,
        )
        .to_ring(constant::CARD::THICKNESS),
    );
    let outline_color = materials.add(Color::linear_rgb(0., 0., 0.));

    let card_theme = {
        let card_color1 = materials.add(Color::linear_rgb(1., 1., 1.));
        let card_color2 = materials.add(Color::linear_rgb(1., 0., 0.));
        let card_color3 = materials.add(Color::linear_rgb(1., 0.64, 0.));
        enum_map! {
            CardType::Demon => card_color1.clone(),
            CardType::Normal => card_color2.clone(),
            CardType::Imp => card_color3.clone()
        }
    };

    let card_visuals = CardVisualInfo {
        card_front_mesh: card_mesh,
        card_outline_mesh: card_outline,
        card_outline_color: outline_color,
        card_theme: CardTheme(card_theme),
    };

    commands.insert_resource(card_visuals.clone());
    card_visuals
}

fn spawn_card(commands: &mut Commands, card_visuals: CardVisualInfo) {
    commands
        .spawn(stack(Vec3::new(-20., 0., 0.)))
        .with_children(|parent| {
            parent
                .spawn(card(
                    Vec3::new(0., 0., 0.),
                    &card_visuals,
                    &CardType::Normal.into(),
                ))
                .observe(create_follow_card)
                .observe(delete_follow_card);
        })
        .observe(drag_drop_combine);

    commands
        .spawn(stack(Vec3::new(20., 0., 0.)))
        .with_children(|parent| {
            parent
                .spawn(card(
                    Vec3::new(0., 0., 0.),
                    &card_visuals,
                    &CardType::Demon.into(),
                ))
                .observe(create_follow_card)
                .observe(delete_follow_card);
        })
        .observe(drag_drop_combine);
}
