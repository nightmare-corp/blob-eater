use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
// use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
struct Player;

// #[derive(Bundle, Debug)]
// pub struct PlayerBundle {
//     pub player: Player,
//     pub transform: Transform,
//     pub global_transform: GlobalTransform,
//     // pub collider: Collider,
//     // pub state: State,
// }

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius: f32 = 10.;
    // Player circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(-radius, 0., 0.)),
            ..default()
        },
        Player,
    ));
    // Shadow object
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.5))), // semi-transparent black for shadow
            transform: Transform::from_translation(Vec3::new(-45., -5., -1.)), // offset and behind the original object
            ..default()
        },
        Player,
    ));
}
fn move_player(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut meshes: Query<(&Mesh2dHandle, &mut Transform, &GlobalTransform), With<Player>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        for (_, mut transform, _) in &mut meshes {
            transform.translation.x = world_position.x;
            transform.translation.y = world_position.y;
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, move_player);
    }
}
