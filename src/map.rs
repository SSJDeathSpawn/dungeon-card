use bevy::prelude::*;

use crate::constant;
pub struct MapPlugin;

#[derive(Component)]
pub struct BoardSettings {
    pub size: Vec2,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map)
            .add_systems(Update, resize_map);

        #[cfg(debug_assertions)]
        app.add_systems(Update, change_board_size_dbg);
    }
}

fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let map_settings = BoardSettings {
        size: Vec2::new(
            constant::MAP::START_FACTOR,
            constant::MAP::START_FACTOR / constant::MAP::RATIO,
        ),
    };

    let mesh = meshes.add(Rectangle::from_size(map_settings.size));
    let mesh_outline =
        meshes.add(Rectangle::from_size(map_settings.size).to_ring(constant::MAP::THICKNESS));
    let mat_color = materials.add(Color::linear_rgb(0.05, 0.05, 0.05));
    let outline_color = materials.add(Color::linear_rgb(0.4, 0.4, 0.4));

    commands
        .spawn((map_settings, Transform::from_xyz(0., 0., -1.)))
        .with_children(|parent| {
            parent.spawn((Mesh2d(mesh), MeshMaterial2d(mat_color)));
            parent.spawn((Mesh2d(mesh_outline), MeshMaterial2d(outline_color)));
        });
}

fn resize_map(
    mut q_board: Query<(&BoardSettings, &mut Mesh2d), Changed<BoardSettings>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok((board_setting, mut mesh)) = q_board.single_mut() else {
        return;
    };
    let mesh_handle = meshes
        .get_mut(&mut mesh.0)
        .expect("No rectangle handle for board");
    *mesh_handle = Rectangle::from_size(board_setting.size).into();
}

#[cfg(debug_assertions)]
fn change_board_size_dbg(mut q_board: Query<&mut BoardSettings>, keys: Res<ButtonInput<KeyCode>>) {
    let Ok(mut board_setting) = q_board.single_mut() else {
        return;
    };
    let should_increase = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
        && keys.just_pressed(KeyCode::Equal);
    let should_decrease = keys.just_pressed(KeyCode::Minus);

    if should_increase {
        board_setting.size.x += 50.;
        board_setting.size.y += 50. / constant::MAP::RATIO;
        println!("Board Setting: {:?}", board_setting.size)
    } else if should_decrease {
        board_setting.size.x -= 50.;
        board_setting.size.y -= 50. / constant::MAP::RATIO;
        println!("Board Setting: {:?}", board_setting.size)
    }
}
