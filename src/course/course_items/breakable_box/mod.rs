use super::*;

mod brerak_effect;
use brerak_effect::*;

pub struct BreakableBoxPlugin;

impl Plugin for BreakableBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FireBreakEffect>()
            .insert_resource(BreakEffect(None))
            .add_systems(Startup, setup_break_effect)
            .add_systems(Update, (breakable_system, handle_break_effect));
    }
}

#[derive(Component)]
pub struct Breakable {
    required_speed: f32,
}
impl Breakable {
    pub fn new(required_speed: f32) -> Self {
        Self { required_speed }
    }
}

pub fn breakable_box_bundle(
    x: f32,
    y: f32,
    required_speed: f32,
    box_size: f32,
    course_materials: &crate::course::CourseMaterials,
) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        Breakable::new(required_speed),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(box_size / 2.0, box_size / 2.0),
        Mesh2d(course_materials.breakable_mesh.clone()),
        MeshMaterial2d(course_materials.breakable_material.clone()),
    )
}


use crate::audio::AudioAssets;

fn breakable_system(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    breakable_query: Query<(Entity, &Breakable)>,
    velocity_query: Query<&Velocity>,
    transform_query: Query<&Transform>,
    mut fire_break_effect: MessageWriter<FireBreakEffect>,
    audio_assets: Res<AudioAssets>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (break_entity, other_entity, breakable) =
                if let Ok((entity, breakable)) = breakable_query.get(*e1) {
                    (entity, *e2, breakable)
                } else if let Ok((entity, breakable)) = breakable_query.get(*e2) {
                    (entity, *e1, breakable)
                } else {
                    continue;
                };

            let v1 = velocity_query.get(break_entity).ok();
            let v2 = velocity_query.get(other_entity).ok();

            let speed = match (v1, v2) {
                (Some(v1), Some(v2)) => (v1.linvel - v2.linvel).length(),
                (Some(v1), None) => v1.linvel.length(),
                (None, Some(v2)) => v2.linvel.length(),
                (None, None) => 0.0,
            };
            if speed >= breakable.required_speed {
                let position = transform_query
                    .get(break_entity)
                    .expect("break_entity don't have transform")
                    .translation;
                fire_break_effect.write(FireBreakEffect(position));
                commands.spawn((
                    AudioPlayer::new(audio_assets.break_sound.clone()),
                    PlaybackSettings::DESPAWN,
                ));
                commands.entity(break_entity).despawn();
            }
        }
    }
}
