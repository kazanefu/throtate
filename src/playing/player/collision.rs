use crate::audio::AudioAssets;
use crate::settings::Settings;
use bevy::audio::Volume;
use bevy::ecs::query::{QueryData, QueryFilter};

use super::*;

pub struct PlayerCollisonPlugin;

impl Plugin for PlayerCollisonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_death, reach_checkpoint, reach_goal).run_if(in_state(GameState::Playing)),
        );
    }
}

fn get_contained_entity<Q: QueryData, F: QueryFilter>(
    e1: Entity,
    e2: Entity,
    query: &Query<Q, F>,
) -> Option<Entity> {
    if query.get(e1).is_ok() {
        Some(e1)
    } else if query.get(e2).is_ok() {
        Some(e2)
    } else {
        None
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_death(
    mut commands: Commands,
    mut player_que: Query<
        (
            &mut DeathCount,
            &mut Transform,
            &TargetCheckPoint,
            &mut Hammer,
        ),
        With<Player>,
    >,
    death_que: Query<&Death>,
    mut collision_event: MessageReader<CollisionEvent>,
    mut hammer_action_writer: MessageWriter<HammerFreeMessage>,
    mut death_writer: MessageWriter<crate::action_effect::FireDeathEffect>,
    audio_assets: Res<AudioAssets>,
    settings: Res<Settings>,
) {
    for &event in collision_event.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let Some(player) = get_contained_entity(e1, e2, &player_que) else {
                continue;
            };
            // unwrap: because it has been confirmed that player_que contains player
            let (mut death_count, mut transform, checkpoint, mut hammer) =
                player_que.get_mut(player).unwrap();
            if get_contained_entity(e1, e2, &death_que).is_none() {
                continue;
            }

            // remove ImpulseJoint: because it is impossible to wrap to checkpoint when detained by ImpulseJoint
            if matches!(hammer.state, HammerState::Spinning) {
                commands.entity(player).remove::<ImpulseJoint>();
                hammer.state = HammerState::Flying;
                hammer_action_writer.write(HammerFreeMessage);
            }

            death_count.count_up();
            death_writer.write(FireDeathEffect(transform.translation));
            commands.spawn((
                AudioPlayer(audio_assets.death.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.audio.se_volume)),
            ));
            transform.translation = checkpoint.position;
        }
    }
}

fn reach_checkpoint(
    mut commands: Commands,
    mut player_que: Query<&mut TargetCheckPoint>,
    mut collision_event: MessageReader<CollisionEvent>,
    checkpoint_que: Query<(&CheckPoint, &Transform)>,
    mut checkpoint_effect: MessageWriter<FireCheckPointEffect>,
    audio_assets: Res<AudioAssets>,
    settings: Res<Settings>,
) {
    for &event in collision_event.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let Some(player) = get_contained_entity(e1, e2, &player_que) else {
                continue;
            };
            let Some(checkpoint_entity) = get_contained_entity(e1, e2, &checkpoint_que) else {
                continue;
            };

            // unwrap: because it has been confirmed that querys contains entitys
            let mut target_checkpoint = player_que.get_mut(player).unwrap();
            let (checkpoint, checkpoint_transform) = checkpoint_que.get(checkpoint_entity).unwrap();

            if checkpoint.priority() >= target_checkpoint.priority {
                target_checkpoint.priority = checkpoint.priority();
                let prev_position = target_checkpoint.position;
                target_checkpoint.position = checkpoint_transform.translation;
                if prev_position != target_checkpoint.position {
                    checkpoint_effect.write(FireCheckPointEffect(target_checkpoint.position));
                    commands.spawn((
                        AudioPlayer(audio_assets.checkpoint.clone()),
                        PlaybackSettings::DESPAWN
                            .with_volume(Volume::Linear(settings.audio.se_volume)),
                    ));
                }
            }
        }
    }
}

fn reach_goal(
    mut commands: Commands,
    mut reach_message: MessageWriter<ReachedGoalMessage>,
    mut collision_event: MessageReader<CollisionEvent>,
    player_query: Query<(), With<Player>>,
    goal_query: Query<(), With<Goal>>,
    audio_assets: Res<AudioAssets>,
    settings: Res<Settings>,
) {
    for &event in collision_event.read() {
        if let CollisionEvent::Started(e1, e2, _) = event
            && get_contained_entity(e1, e2, &player_query).is_some()
            && get_contained_entity(e1, e2, &goal_query).is_some()
        {
            reach_message.write(ReachedGoalMessage);
            commands.spawn((
                AudioPlayer(audio_assets.goal.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.audio.se_volume)),
            ));
            println!("Player reached the goal!");
        }
    }
}
