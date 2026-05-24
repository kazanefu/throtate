use super::*;

pub struct WarpHolePlugin;

impl Plugin for WarpHolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_warp, update_warp_cooldown).run_if(in_state(crate::state::GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct WarpPortal {
    pub pair_x: f32,
    pub pair_y: f32,
}

#[derive(Component)]
pub struct WarpCooldown {
    pub timer: Timer,
}

const WARP_COOLDOWN_SECS: f32 = 0.5;

impl WarpPortal {
    pub fn new(pair_x: f32, pair_y: f32) -> Self {
        Self { pair_x, pair_y }
    }
}

pub fn warp_portal_bundle(
    x: f32,
    y: f32,
    pair_x: f32,
    pair_y: f32,
    box_size: f32,
    course_materials: &crate::course::CourseMaterials,
) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        WarpPortal::new(pair_x, pair_y),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(box_size / 2.0, box_size / 2.0),
        Sensor,
        Mesh2d(course_materials.warp_mesh.clone()),
        MeshMaterial2d(course_materials.warp_material.clone()),
    )
}

fn get_contained_entity<Q: bevy::ecs::query::QueryData, F: bevy::ecs::query::QueryFilter>(
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

fn handle_warp(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    warp_portal_query: Query<&WarpPortal>,
    mut player_query: Query<
        (&mut Transform, &mut crate::hammer::definition::Hammer),
        Without<WarpCooldown>,
    >,
    player_check: Query<Entity, With<crate::playing::Player>>,
    mut hammer_action_writer: MessageWriter<crate::hammer::definition::HammerFreeMessage>,
    audio_assets: Res<crate::audio::AudioAssets>,
) {
    for &event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Check if one entity is a warp portal
            let Some(portal_entity) = get_contained_entity(e1, e2, &warp_portal_query) else {
                continue;
            };

            // Check if the other entity is the player
            let player_entity = if e1 == portal_entity { e2 } else { e1 };

            if player_check.get(player_entity).is_err() {
                continue;
            }

            let Ok((mut player_transform, mut hammer)) = player_query.get_mut(player_entity) else {
                continue;
            };

            // Get the warp portal data
            let portal = warp_portal_query.get(portal_entity).unwrap();

            // Remove ImpulseJoint if spinning (same as death/checkpoint behavior)
            if matches!(
                hammer.state,
                crate::hammer::definition::HammerState::Spinning
            ) {
                commands.entity(player_entity).remove::<ImpulseJoint>();
                hammer.state = crate::hammer::definition::HammerState::Flying;
                hammer_action_writer.write(crate::hammer::definition::HammerFreeMessage);
            }

            // Teleport the player to the paired portal
            player_transform.translation.x = portal.pair_x;
            player_transform.translation.y = portal.pair_y;

            // Play warp sound
            commands.spawn((
                AudioPlayer::new(audio_assets.warp.clone()),
                PlaybackSettings::DESPAWN,
            ));

            // Add cooldown to prevent immediate re-warping
            commands.entity(player_entity).insert(WarpCooldown {
                timer: Timer::from_seconds(WARP_COOLDOWN_SECS, TimerMode::Once),
            });
        }
    }
}

fn update_warp_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    mut cooldown_query: Query<(Entity, &mut WarpCooldown)>,
) {
    for (entity, mut cooldown) in &mut cooldown_query {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.just_finished() {
            commands.entity(entity).remove::<WarpCooldown>();
        }
    }
}
