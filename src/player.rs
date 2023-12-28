use bevy::prelude::*;

use crate::{
    characters::{CharacterBundle, CharacterData},
    ui::{LevelText, UIPlugin},
};
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
struct Player;

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius: f32 = 10.;
    // Player circle
    commands
        .spawn((
            CharacterBundle {
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            CharacterData { radius },
            Player,
        ))
        .insert((
            Collider::ball(radius * 0.96),
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
) {
    move_player(windows, camera_q, &mut character_data_player);
    handle_collision(
        collision_events,
        character_data,
        character_data_player,
        commands,
        level_text_query,
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
///
fn handle_collision(
    mut collision_events: EventReader<CollisionEvent>,
    character_data: Query<(&mut CharacterData, &mut Transform), Without<Player>>,
    mut character_data_player: Query<(&mut CharacterData, &mut Transform), With<Player>>,
    mut commands: Commands,
    //update ui
    mut level_text_query: Query<&mut Text, With<LevelText>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            // On collision
            CollisionEvent::Started(player, npc, _) => {
                println!("Player {:?} collided with NPC {:?}", player, npc);
                // This looks awkward. Is there a better way to do this?
                let player_radius = character_data_player
                    .get(*player)
                    .map(|(player_data, _)| player_data.radius)
                    .unwrap_or_default();
                let npc_radius = character_data
                    .get(*npc)
                    .map(|(npc_data, _)| npc_data.radius)
                    .unwrap_or_default();

                println!(
                    "Player radius: {}, Npc radius {}",
                    player_radius, npc_radius
                );
                //player eats npc
                if player_radius > npc_radius {
                    println!("Player is bigger than NPC");
                    commands.entity(*npc).despawn();
                    //TODO player would never be in here.
                    if let Ok((mut player_data, mut player_transform)) =
                        character_data_player.get_mut(*player)
                    {
                        player_data.radius += npc_radius;
                        // player_transform.scale
                        println!("Player radius: {}", player_data.radius);
                        //TODO how to update the mesh?
                        player_transform.scale = player_transform.scale * 1.05;
                        for mut text in &mut level_text_query {
                            text.sections[1].value = format!("{}", player_data.radius);
                        }
                    }
                } else {
                    println!("GAME OVER");
                    // Despawn player hierarchy
                    commands.entity(*player).despawn_recursive();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
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
