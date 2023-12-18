use std::f32::consts::E;

use crate::characters::CharacterBundle;
use bevy::prelude::*;
use rand::{distributions::Distribution, Rng};

const NPC_COUNT: usize = 100;

#[derive(Component)]
pub struct Npc {
    // pub direction_id: u8,
    pub speed: f32,
    pub direction: Vec2,
}
impl Npc {
    fn new(direction: Vec2) -> Self {
        //TODO maybe no need to normalize..
        let direction = direction.normalize();
        Self {
            // direction_id: rand::thread_rng().gen_range(0..8),
            speed: rand::thread_rng().gen_range(50.0..300.0),
            direction,
        }
    }
}
pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (npc_movement, npc_spawn));
    }
}
#[derive(Copy, Clone)]
enum SpawnOrigin {
    Left,
    Right,
    Top,
    Bottom,
}
//standard distribution for spawn origin so we can use rng.gen()
impl Distribution<SpawnOrigin> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SpawnOrigin {
        match rng.gen_range(0..3) {
            0 => SpawnOrigin::Left,
            1 => SpawnOrigin::Right,
            2 => SpawnOrigin::Top,
            3 => SpawnOrigin::Bottom,
            _ => SpawnOrigin::Left,
        }
    }
}
///right now reverses the vector
fn point_to_direction(location: Vec2) -> Vec2 {
    //direction
    // Vec2::new(0., 0.)
    //first reverse the vector
    let direction = -location;

    return direction;
}
fn calc_npc_spawn(padding: f32, bounds: Vec2) -> (Vec2, Vec2) {
    let mut rng = rand::thread_rng();

    //grab the middle of both sides of the screen
    let x = bounds.x / 2.0;
    let y = bounds.y / 2.0;

    let origin: SpawnOrigin = rng.gen();
    println!("origin: {:?}", origin as u8);

    let mut location = Vec2::ZERO;
    let mut direction = Vec2::ZERO;
    match origin {
        SpawnOrigin::Left => {
            location.x = -x - padding;
            location.y = rng.gen_range(-y..y);

            //TODO randomize direction towards center/inner screen
        }
        SpawnOrigin::Right => {
            location.x = x + padding;
            location.y = rng.gen_range(-y..y);
        }
        SpawnOrigin::Top => {
            location.y = y - padding;
            location.x = rng.gen_range(-x..x);
        }
        SpawnOrigin::Bottom => {
            location.y = -y + padding;
            location.x = rng.gen_range(-x..x);
        }
    }
    direction = point_to_direction(location);
    (location, direction)
}

fn npc_spawn(
    mut commands: Commands,
    query: Query<Entity, With<Npc>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let window: &Window = windows.single();
    let bounds: Vec2 = Vec2::new(window.width(), window.height());
    let (location, direction) = calc_npc_spawn(10., bounds);
    // TODO radius depends on current score.
    let radius: f32 = 10.;
    let count = query.iter().count();
    // TODO color radomized
    let color = Color::RED;
    // Direction random.

    if count < NPC_COUNT {
        commands.spawn((
            CharacterBundle {
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new(-radius, location.x, location.y)),
                ..default()
            },
            Npc::new(direction),
        ));
    }
}
fn npc_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut npc_position: Query<(Entity, &mut Transform, &Npc)>,
    mut windows: Query<&Window>,
) {
    let window = windows.single();
    let bounds = (window.width(), window.height());
    for (entity, mut transform, npc) in &mut npc_position {
        let direction = npc.direction;

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
