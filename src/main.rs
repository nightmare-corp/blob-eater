use bevy::{
    app::{App, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Res},
    input::Input,
    prelude::*,
    DefaultPlugins,
};
use bevy_rapier2d::prelude::*;

use characters::CharacterData;
use npcs::NpcPlugin;
use player::player_setup;
use ui::LevelText;
mod characters;
mod npcs;
mod player;
mod ui;

//TODO UI for debug purposes, e.g. FPS counter and other data.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((player::PlayerPlugin, NpcPlugin))
        .add_plugins((RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),))
        .add_systems(Startup, setup_foundation)
        // .add_systems(Update, exit_game)
        .add_systems(PreUpdate, restart_game)
        .run();
}

fn restart_game(
    mut commands: Commands,
    mut character_data: Query<(Entity, &mut CharacterData)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level_text_query: Query<&mut Text, With<LevelText>>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Despawn all characters including player
        for (entity, _) in &mut character_data {
            commands.entity(entity).despawn_recursive();
        }
        info!("Restarting game");
        // Spawn fresh player
        player_setup(commands, meshes, materials, level_text_query);
    }
}

fn setup_foundation(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
