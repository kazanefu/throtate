use bevy::prelude::*;
use serde::Deserialize;
pub mod course_items;
mod load_course;
pub mod spawn;
pub use spawn::*;

use crate::state::GameState;

pub struct CoursePlugin;

impl Plugin for CoursePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(load_course::CourseLoadPlugin)
            .insert_resource(CourseListResource::default())
            .add_plugins(course_items::turret::TurretPlugin)
            .add_plugins(course_items::breakable_box::BreakableBoxPlugin)
            .add_plugins(course_items::speedup::SpeedUpPlugin)
            .add_plugins(course_items::spin_veladd_time::SpinVelAddTimePlugin)
            .add_message::<SpawnCourseMessage>() //init_courses_list_resource)
            .add_systems(Startup, setup_course_materials)
            .add_systems(Update, spawn_course_from_id);
    }
}

#[derive(Resource)]
pub struct CourseMaterials {
    pub breakable_mesh: Handle<Mesh>,
    pub breakable_material: Handle<crate::materials::BreakableMaterial>,
    pub checkpoint_mesh: Handle<Mesh>,
    pub checkpoint_material: Handle<crate::materials::CheckpointMaterial>,
    pub death_mesh: Handle<Mesh>,
    pub death_material: Handle<crate::materials::DeathMaterial>,
    pub goal_mesh: Handle<Mesh>,
    pub goal_material: Handle<crate::materials::GoalMaterial>,
    pub turret_mesh: Handle<Mesh>,
    pub turret_material: Handle<crate::materials::TurretMaterial>,
    pub bullet_mesh: Handle<Mesh>,
    pub bullet_material: Handle<crate::materials::BulletMaterial>,
    pub speedup_mesh: Handle<Mesh>,
    pub speedup_material: Handle<crate::materials::SpeedupMaterial>,
}

use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
struct CourseMaterialAssets<'w, 's> {
    meshes: ResMut<'w, Assets<Mesh>>,
    breakable_materials: ResMut<'w, Assets<crate::materials::BreakableMaterial>>,
    checkpoint_materials: ResMut<'w, Assets<crate::materials::CheckpointMaterial>>,
    death_materials: ResMut<'w, Assets<crate::materials::DeathMaterial>>,
    goal_materials: ResMut<'w, Assets<crate::materials::GoalMaterial>>,
    turret_materials: ResMut<'w, Assets<crate::materials::TurretMaterial>>,
    bullet_materials: ResMut<'w, Assets<crate::materials::BulletMaterial>>,
    speedup_materials: ResMut<'w, Assets<crate::materials::SpeedupMaterial>>,
    _marker: std::marker::PhantomData<&'s ()>,
}

fn setup_course_materials(
    mut commands: Commands,
    mut assets: CourseMaterialAssets,
    config: Res<crate::config::GameConfig>,
) {
    let box_size = config.course.one_box_size;
    commands.insert_resource(CourseMaterials {
        breakable_mesh: assets.meshes.add(Rectangle::new(box_size, box_size)),
        breakable_material: assets
            .breakable_materials
            .add(crate::materials::BreakableMaterial::default()),
        checkpoint_mesh: assets.meshes.add(Rectangle::new(box_size, box_size)),
        checkpoint_material: assets
            .checkpoint_materials
            .add(crate::materials::CheckpointMaterial::default()),
        death_mesh: assets.meshes.add(Rectangle::new(box_size, box_size)),
        death_material: assets
            .death_materials
            .add(crate::materials::DeathMaterial::default()),
        goal_mesh: assets.meshes.add(Rectangle::new(box_size, box_size)),
        goal_material: assets
            .goal_materials
            .add(crate::materials::GoalMaterial::default()),
        turret_mesh: assets.meshes.add(Rectangle::new(box_size, box_size)),
        turret_material: assets
            .turret_materials
            .add(crate::materials::TurretMaterial::default()),
        bullet_mesh: assets
            .meshes
            .add(Rectangle::new(box_size / 2.0, box_size / 2.0)),
        bullet_material: assets
            .bullet_materials
            .add(crate::materials::BulletMaterial::default()),
        speedup_mesh: assets.meshes.add(Rectangle::new(box_size, box_size)),
        speedup_material: assets
            .speedup_materials
            .add(crate::materials::SpeedupMaterial::new(Color::srgb(
                0.5, 1.0, 1.0,
            ))),
    });
}

#[derive(Resource, Default)]
pub struct CourseListResource(pub Vec<(CourseEntry, Course)>);

#[derive(Component)]
pub struct CourseID {
    #[allow(unused)]
    id: usize,
}

impl CourseID {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

#[derive(Deserialize, Default)]
pub struct Course {
    pub entities: Vec<EntityData>,
}

#[derive(Deserialize)]
pub struct EntityData {
    pub x: f32,
    pub y: f32,
    pub kind: EntityKind,
}

#[derive(Deserialize)]
pub enum EntityKind {
    Ground {
        width: f32,
        height: f32,
    },
    Turret {
        interval: f32,
        rotation: f32,
        bullet_lifetime: Option<f32>,
    },
    Breakable {
        required_speed: f32,
    },
    Death,
    Checkpoint {
        priority: u32,
    },
    Goal,
    Text {
        sentence: String,
    },
    Dynamic {
        width: Option<f32>,
        height: Option<f32>,
        gravity_scale: Option<f32>,
        linear_damping: Option<f32>,
        angular_damping: Option<f32>,
        density_scale: Option<f32>,
        restitution_coefficient: Option<f32>,
    },
    SpeedUp {
        rate: f32,
    },
    SpinVelAddWithTime {
        value: f32,
        time: f32,
    },
}

#[derive(Deserialize, Default)]
pub struct CourseList(pub Vec<CourseEntry>);

#[derive(Deserialize, Clone)]
pub struct CourseEntry {
    pub id: usize,
    pub name: String,
    pub path: String,
}
