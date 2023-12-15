use crate::characters::CharacterBundle;
use bevy::prelude::*;
use rand::Rng;

const NPC_COUNT: usize = 100;

#[derive(Component)]
pub struct Npc {
    pub direction_id: u8,
    pub speed: f32,
}
impl Npc {
    fn new() -> Self {
        Self {
            direction_id: rand::thread_rng().gen_range(0..8),
            speed: rand::thread_rng().gen_range(50.0..300.0),
        }
    }
}
impl Default for Npc {
    fn default() -> Self {
        Self::new()
    }
}
pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, npc_setup)
            .add_systems(Update, (npc_movement, npc_spawn));
    }
}

fn direction_to_vec(dir_id: u8) -> Vec2 {
    match dir_id {
        0 => Vec2::new(0., 1.),
        1 => Vec2::new(1., 1.),
        2 => Vec2::new(1., 0.),
        3 => Vec2::new(1., -1.),
        4 => Vec2::new(0., -1.),
        5 => Vec2::new(-1., -1.),
        6 => Vec2::new(-1., 0.),
        7 => Vec2::new(-1., 1.),
        _ => Vec2::new(0., 1.),
    }
}

fn npc_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius: f32 = 10.;
    //npcs
    commands.spawn((
        CharacterBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(-radius, 0., 0.)),
            ..default()
        },
        Npc::default(),
    ));
}
fn npc_spawn(
    mut commands: Commands,
    query: Query<Entity, With<Npc>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO radius depends on current score.
    let radius: f32 = 10.;
    let count = query.iter().count();
    // TODO color radomized
    let color = Color::RED;
    let location = Vec2::new(0., 0.);
    // Direction random.

    if count < NPC_COUNT {
        commands.spawn((
            CharacterBundle {
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new(-radius, location.x, location.y)),
                ..default()
            },
            Npc::default(),
        ));
    }
}
fn npc_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut npc_position: Query<(Entity, &mut Transform, &Npc)>,
) {
    let bounds = (2000., 2000.);
    for (entity, mut transform, npc) in &mut npc_position {
        let direction = direction_to_vec(npc.direction_id.clone()).normalize();

        transform.translation.x += direction.x * time.delta_seconds() * npc.speed;
        transform.translation.y += direction.y * time.delta_seconds() * npc.speed;

        if transform.translation.y > bounds.0 {
            commands.entity(entity).despawn();
        } else if transform.translation.y < -bounds.0 {
            commands.entity(entity).despawn();
        }

        if transform.translation.x > bounds.1 {
            commands.entity(entity).despawn();
        } else if transform.translation.x < -bounds.1 {
            commands.entity(entity).despawn();
        }
    }
}
