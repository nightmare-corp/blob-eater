use bevy::prelude::*;

use crate::{
    characters::{CharacterBundle, CharacterData},
    ui::{LevelText, UIPlugin},
};
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
struct Player;

#[derive(Resource, Debug)]
pub struct PlayerLevel(u32);

impl PlayerLevel {
    // Getter
    pub fn level(&self) -> u32 {
        self.0
    }
    // Adds one
    pub fn plus(&mut self) {
        self.0 += 1;
    }
}

pub fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level_text_query: Query<&mut Text, With<LevelText>>,
) {
    commands.insert_resource(PlayerLevel(0));
    for mut text in &mut level_text_query {
        text.sections[1].value = format!("{}", 0);
    }
    // Player circle
    commands
        .spawn((
            CharacterBundle {
                mesh: meshes
                    .add(shape::Circle::new(PLAYER_START_RADIUS).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            CharacterData {
                radius: PLAYER_START_RADIUS,
            },
            Player,
        ))
        .insert((
            Collider::ball(PLAYER_START_RADIUS * 0.96),
            //TODO why does it only work when RigidBodyType::Dynamic?
            RigidBody::Dynamic,
            GravityScale(0.0),
        ))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS);
}

//TODO I need feedback on this whole function.
fn player_frame(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    // mut meshes: Query<(&Mesh2dHandle, &mut Transform, &GlobalTransform), With<Player>>,
    collision_events: EventReader<CollisionEvent>,
    //TODO how to get transform?
    character_data: Query<(&mut CharacterData, &mut Transform), Without<Player>>,
    mut character_data_player: Query<(&mut CharacterData, &mut Transform), With<Player>>,
    commands: Commands,
    level_text_query: Query<&mut Text, With<LevelText>>,
    player_level: ResMut<PlayerLevel>,
) {
    move_player(windows, camera_q, &mut character_data_player);
    handle_collision(
        collision_events,
        character_data,
        character_data_player,
        commands,
        level_text_query,
        player_level,
    );
}

fn move_player(
    windows: Query<'_, '_, &Window>,
    camera_q: Query<'_, '_, (&Camera, &GlobalTransform)>,
    character_data_player: &mut Query<'_, '_, (&mut CharacterData, &mut Transform), With<Player>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        for (_, mut transform_) in character_data_player {
            transform_.translation.x = world_position.x;
            transform_.translation.y = world_position.y;
        }
    }
}
const PLAYER_START_RADIUS: f32 = 10.;
fn handle_collision(
    mut collision_events: EventReader<CollisionEvent>,
    character_data: Query<(&mut CharacterData, &mut Transform), Without<Player>>,
    mut character_data_player: Query<(&mut CharacterData, &mut Transform), With<Player>>,
    mut commands: Commands,
    //update ui
    mut level_text_query: Query<&mut Text, With<LevelText>>,
    mut player_level: ResMut<PlayerLevel>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            //Possible issue is that the tuple values are switched arround aga,
            // On collision
            CollisionEvent::Started(col1, col2, _) => {
                let player_radius = character_data_player.single_mut().0.radius;

                info!("player_radius: {}", player_radius);
                let (npc_radius, npc_id, player_id) = character_data
                    .get(*col2)
                    .map(|(npc_data, _)| (npc_data.radius, col2, col1))
                    .unwrap_or_else(|_| {
                        info!("Tried number 2");
                        character_data
                            .get(*col1)
                            .map(|(npc_data, _)| (npc_data.radius, col1, col2))
                            .unwrap_or_else(|_| {
                                error!("No radius found for npc");
                                //return
                                return (default(), col1, col2);
                            })
                    });
                // Player eats npc
                // Player gets a small boost.
                info!(
                    "Player radius: {}, NPC radius: {}",
                    player_radius, npc_radius
                );
                if player_radius + 0.5 > npc_radius {
                    commands.entity(*npc_id).despawn();
                    player_levels(
                        &mut character_data_player,
                        player_id,
                        &mut player_level,
                        &mut level_text_query,
                    );
                } else {
                    //TODO game over overlay message.
                    info!("GAME OVER");
                    // Despawn player hierarchy
                    commands.entity(*player_id).despawn_recursive();
                    //TODO click to restart game.
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn player_levels(
    character_data_player: &mut Query<'_, '_, (&mut CharacterData, &mut Transform), With<Player>>,
    player: &Entity,
    player_level: &mut ResMut<'_, PlayerLevel>,
    level_text_query: &mut Query<'_, '_, &mut Text, With<LevelText>>,
) {
    if let Ok((mut player_data, mut player_transform)) = character_data_player.get_mut(*player) {
        player_level.plus();
        let level = player_level.level().clone();
        player_data.radius = level as f32 * PLAYER_START_RADIUS * 0.1 + PLAYER_START_RADIUS;
        player_transform.scale = Vec3::splat(1.0 + level as f32 * 0.1);
        for mut text in level_text_query {
            text.sections[1].value = format!("{}", level);
        }
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UIPlugin)
            .add_systems(Startup, player_setup)
            .add_systems(Update, player_frame);
    }
}
