use super::*;

pub struct BreakableBoxPlugin;

impl Plugin for BreakableBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, breakable_system);
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

pub fn breakable_box_bundle(x: f32, y: f32, required_speed: f32) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        Breakable::new(required_speed),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(ONE_BOX_SIZE / 2.0, ONE_BOX_SIZE / 2.0),
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.2),
            custom_size: Some(Vec2::new(ONE_BOX_SIZE, ONE_BOX_SIZE)),
            ..default()
        },
    )
}

fn breakable_system(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    breakable_query: Query<(Entity, &Breakable)>,
    velocity_query: Query<&Velocity>,
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
                commands.entity(break_entity).despawn();
            }
        }
    }
}
