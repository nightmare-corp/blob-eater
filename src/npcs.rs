use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::characters::CharacterBundle;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, npc_setup)
            .add_systems(Update, npc_movement);
    }
}
#[derive(Component)]
enum Direction {
    Up,
    Down,
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
        Direction::Up,
    ));
}

fn npc_movement(time: Res<Time>, mut npc_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut dir, mut transform) in &mut npc_position {
        match *dir {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *dir = Direction::Down;
        } else if transform.translation.y < -200. {
            *dir = Direction::Up;
        }
    }
}
