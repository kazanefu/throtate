use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod bullet;

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (turret_shot, bullet::tick_bullets));
    }
}

#[derive(Component)]
pub struct Turret {
    pub bullet_lifetime: f32,
}

#[derive(Component)]
pub struct BulletPool {
    pub bullets: Vec<Entity>,
}

const BULLET_LIFE_TIME: f32 = 3.0;
const BULLET_SPEED: f32 = 500.0;

pub struct TurretSpawnParams<'a> {
    pub x: f32,
    pub y: f32,
    pub interval: f32,
    pub rotation: f32,
    pub bullet_lifetime: Option<f32>,
    pub box_size: f32,
    pub course_materials: &'a crate::course::CourseMaterials,
}

pub fn spawn_turret<'a, 'b>(
    commands: &'b mut Commands,
    params: TurretSpawnParams<'a>,
) -> EntityCommands<'b> {
    let bullet_lifetime = params.bullet_lifetime.unwrap_or(BULLET_LIFE_TIME);
    let pool_size = (bullet_lifetime / params.interval) as usize + 1;
    let mut bullets = Vec::with_capacity(pool_size);
    for _ in 0..pool_size {
        bullets.push(bullet::spawn_inactive_bullet(commands, params.box_size, params.course_materials));
    }

    commands.spawn((
        Transform {
            translation: Vec3::new(params.x, params.y, 0.0),
            rotation: Quat::from_rotation_z(params.rotation),
            scale: Vec3::ONE,
        },
        crate::utils::Interval {
            time: 0.0,
            interval: params.interval,
        },
        Turret {
            bullet_lifetime,
        },
        BulletPool { bullets },
        RigidBody::Fixed,
        Collider::cuboid(params.box_size / 2.0, params.box_size / 2.0),
        Mesh2d(params.course_materials.turret_mesh.clone()),
        MeshMaterial2d(params.course_materials.turret_material.clone()),
    ))
}

fn turret_shot(
    mut commands: Commands,
    mut turret_query: Query<(
        &Transform,
        &mut crate::utils::Interval,
        &Turret,
        &mut BulletPool,
    )>,
    mut bullet_query: Query<(
        &mut bullet::TurretBullet,
        &mut Transform,
        &mut Velocity,
        &mut CollisionGroups,
        &mut Visibility,
    ), Without<Turret>>,
    config: Res<crate::config::GameConfig>,
    course_materials: Res<crate::course::CourseMaterials>,
) {
    let box_size = config.course.one_box_size;
    for (turret_transform, mut turret_interval, turret, mut bullet_pool) in &mut turret_query {
        if turret_interval.is_ready() {
            turret_interval.reset();

            // Find an inactive bullet from the pool
            let mut chosen_bullet = None;
            for &bullet_entity in &bullet_pool.bullets {
                let is_inactive = bullet_query
                    .get(bullet_entity)
                    .map(|(bullet, _, _, _, _)| !bullet.is_active)
                    .unwrap_or(false);

                if is_inactive {
                    chosen_bullet = Some(bullet_entity);
                    break;
                }
            }

            let bullet_entity = match chosen_bullet {
                Some(entity) => entity,
                None => {
                    // dynamically spawn a new bullet and add to the pool
                    let entity = bullet::spawn_inactive_bullet(&mut commands, box_size, &course_materials);
                    bullet_pool.bullets.push(entity);
                    entity
                }
            };

            // Activate the bullet
            if let Ok((
                mut bullet,
                mut transform,
                mut velocity,
                mut collision_groups,
                mut visibility,
            )) = bullet_query.get_mut(bullet_entity)
            {
                bullet.is_active = true;
                bullet.remaining_lifetime = turret.bullet_lifetime;

                let dir = (turret_transform.rotation * Vec3::X).truncate();

                transform.translation = turret_transform.translation
                    + turret_transform.rotation * Vec3::X * box_size;
                transform.rotation = turret_transform.rotation;

                *velocity = Velocity {
                    linvel: dir * BULLET_SPEED,
                    angvel: 0.0,
                };
                *collision_groups = CollisionGroups::new(Group::ALL, Group::ALL);
                *visibility = Visibility::Inherited;
            }
        }
    }
}
