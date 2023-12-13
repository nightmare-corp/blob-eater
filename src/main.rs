//! Shows how to render simple primitive shapes with a single color.

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_location)
        .run();
}
fn mouse_location(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut meshes: Query<(&Mesh2dHandle, &mut Transform, &GlobalTransform), With<PlayerCharacter>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        for (_, mut transform, _) in &mut meshes {
            //moves player to the right\
            transform.translation.x = world_position.x;
            transform.translation.y = world_position.y;
        }
    }
}
#[derive(Component)]
struct PlayerCharacter;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //camera
    commands.spawn(Camera2dBundle::default());

    // Player circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        },
        PlayerCharacter,
    ));
    // Shadow object
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.5))), // semi-transparent black for shadow
            transform: Transform::from_translation(Vec3::new(-45., -5., -1.)), // offset and behind the original object
            ..default()
        },
        PlayerCharacter,
    ));
}
