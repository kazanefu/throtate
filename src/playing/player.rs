use crate::{
    course::course_items::{
        checkpoint::{self, CheckPoint},
        death_box::Death,
        goal::Goal,
    },
    hammer::definition::{Hammer, HammerFreeMessage, HammerState},
    state::{GameState, RunningState},
};
use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_death,reach_checkpoint).run_if(in_state(GameState::Playing).and(in_state(RunningState::Running))),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct DeathCount(pub u32);

#[derive(Component, Default)]
pub struct TargetCheckPoint {
    pub position: Vec3,
    pub priority: u32,
}

fn handle_death(
    mut commands: Commands,
    mut hammer_query: Query<&mut Hammer>,
    mut player_query: Query<
        (&mut DeathCount, &mut Transform, &TargetCheckPoint, Entity),
        With<Player>,
    >,
    death_query: Query<&Death>,
    mut collision_event: MessageReader<CollisionEvent>,
    mut hammer_action_writer: MessageWriter<HammerFreeMessage>,
) {
    let mut player = player_query
        .single_mut()
        .expect("found none or multiple player in the world");
    for event in collision_event.read() {
        match *event {
            CollisionEvent::Started(e1, e2, _) => {
                if (player.3 == e1 && death_query.get(e2).is_ok())
                    || (player.3 == e2 && death_query.get(e1).is_ok())
                {
                    hammer_action_writer.write(HammerFreeMessage);
                    for mut hammer in &mut hammer_query {
                        if matches!(hammer.state, HammerState::Spinning) {
                            commands.entity(player.3).remove::<ImpulseJoint>();
                            hammer.state = HammerState::Flying;
                        }
                    }
                    player.0.0 += 1;
                    player.1.translation = player.2.position;
                }
            }
            CollisionEvent::Stopped(_e1, _e2, _) => {}
        }
    }
}

const DUMMY_CHECK_POINT: (CheckPoint, Transform) = (
    CheckPoint::ZERO,
    Transform {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    },
);

fn reach_checkpoint(
    mut player_query: Query<(Entity, &mut TargetCheckPoint)>,
    mut collision_event: MessageReader<CollisionEvent>,
    checkpoint_query: Query<(&CheckPoint, &Transform)>,
) {
    for &event in collision_event.read() {
        for (player_entity, mut target_checkpoint) in &mut player_query {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if e1 != player_entity && e2 != player_entity {break;}
                let checkpoint = checkpoint_query.get(e1).unwrap_or(
                    checkpoint_query
                        .get(e2)
                        .unwrap_or((&DUMMY_CHECK_POINT.0, &DUMMY_CHECK_POINT.1)),
                );
                if checkpoint.0.priority() >= target_checkpoint.priority {
                    target_checkpoint.priority = checkpoint.0.priority();
                    target_checkpoint.position = checkpoint.1.translation;
                }
            }
        }
    }
}
