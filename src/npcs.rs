use crate::{
    characters::{CharacterBundle, CharacterData},
    player::PlayerLevel,
};
use bevy::prelude::*;
use bevy_rapier2d::geometry::{Collider, Sensor};
use rand::{distributions::Distribution, Rng};

const NPC_COUNT: usize = 40;

#[derive(Component)]
pub struct Npc {
    pub speed: f32,
    pub direction: Vec2,
}
impl Npc {
    fn new(mut direction: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let random_number = rand::thread_rng().gen_range(0. ..8.);
        if rng.gen() {
            direction.x = direction.x / random_number;
        } else {
            direction.y = direction.y / random_number;
        }
        let direction = direction.normalize();
        Self {
            //TODO I want a distrubution here
            speed: rand::thread_rng().gen_range(15.0..300.0),
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
// Standard distribution for spawn origin so we can use rng.gen()
impl Distribution<SpawnOrigin> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SpawnOrigin {
        match rng.gen_range(0..4) {
            0 => SpawnOrigin::Left,
            1 => SpawnOrigin::Right,
            2 => SpawnOrigin::Top,
            3 => SpawnOrigin::Bottom,
            _ => SpawnOrigin::Left,
        }
    }
}
/// Simply reverses the vector
fn point_to_direction(location: Vec2) -> Vec2 {
    let direction = -location;

    return direction;
}
/// Calculates the spawn location and direction
/// Padding is the distance from the edge of the screen, in this case the scale
/// Bounds is the size of the screen
fn calc_npc_spawn(padding: f32, bounds: Vec2) -> (Vec2, Vec2) {
    let mut rng = rand::thread_rng();

    // Grab the middle of both sides of the screen
    let x = bounds.x / 2.0;
    let y = bounds.y / 2.0;

    let origin: SpawnOrigin = rng.gen();

    let mut location = Vec2::ZERO;
    match origin {
        SpawnOrigin::Left => {
            location.x = -x - padding;
            location.y = rng.gen_range(-y..y);
        }
        SpawnOrigin::Right => {
            location.x = x + padding;
            location.y = rng.gen_range(-y..y);
        }
        SpawnOrigin::Top => {
            location.y = y + padding;
            location.x = rng.gen_range(-x..x);
        }
        SpawnOrigin::Bottom => {
            location.y = -y - padding;
            location.x = rng.gen_range(-x..x);
        }
    }
    let direction = point_to_direction(location);
    (location, direction)
}
/// Calculates the radius and level of the npc based on the current player level
fn radius_from_level(player_level: u32) -> f32 {
    // an idea
    // if player_level < 10 {
    //     let level = player_level as f32;
    //     let mut rng = rand::thread_rng();
    //     return rng.gen_range(7.0..level + 60.0);    // }

    // another idea throw in a few giant circles. (let's use a distrubtion for this)

    let level = player_level as f32;
    let mut rng = rand::thread_rng();
    rng.gen_range(7.0..level + 40.0)
}
fn npc_spawn(
    mut commands: Commands,
    query: Query<Entity, With<Npc>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    player_level: Res<PlayerLevel>,
) {
    let mut rng = rand::thread_rng();
    let window: &Window = windows.single();
    let bounds: Vec2 = Vec2::new(window.width(), window.height());
    let radius = radius_from_level(player_level.level() + 1);
    let (location, direction) = calc_npc_spawn(10., bounds);

    let count = query.iter().count();
    // Color radomized
    let color: Color = Color::rgb(
        rng.gen::<f32>(), // Red
        rng.gen::<f32>(), // Green
        rng.gen::<f32>(), // Blue
    );
    if count < NPC_COUNT {
        commands
            .spawn((
                CharacterBundle {
                    mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                    material: materials.add(ColorMaterial::from(color)),
                    transform: Transform::from_translation(Vec3::new(
                        location.x, location.y, -radius,
                    )),
                    ..default()
                },
                Npc::new(direction),
                CharacterData { radius },
            ))
            .insert(Collider::ball(radius * 0.96))
            .insert(Sensor);
    }
}
fn npc_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut npc_position: Query<(Entity, &mut Transform, &Npc)>,
    windows: Query<&Window>,
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
