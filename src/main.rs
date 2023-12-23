use bevy::{
    app::{App, AppExit, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        event::EventWriter,
        system::{Commands, Res},
    },
    input::{keyboard::KeyCode, Input},
    DefaultPlugins,
};
use bevy_rapier2d::prelude::*;

use npcs::NpcPlugin;
mod characters;
mod npcs;
mod player;

//TODO UI for debug purposes, e.g. FPS counter and other data.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((player::PlayerPlugin, NpcPlugin))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, setup_foundation)
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup_foundation(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
