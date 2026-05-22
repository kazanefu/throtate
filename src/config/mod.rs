use bevy::prelude::*;

mod input;

#[derive(Resource, Default, Clone)]
pub struct GameConfig {
    pub input: input::InputSetting,
    pub hammer: HammerConfig,
    pub course: CourseConfig,
}

#[derive(Clone)]
pub struct HammerConfig {
    pub handle_offset: Vec2,
    pub size: f32,
    pub spin_velocity: f32,
    pub spin_stiffness: f32,
    pub gravity_scale: f32,
    pub restitution_coefficient: f32,
}

impl Default for HammerConfig {
    fn default() -> Self {
        Self {
            handle_offset: Vec2::new(-80.0, 0.0),
            size: 20.0,
            spin_velocity: 100.0,
            spin_stiffness: 0.1,
            gravity_scale: 1.0,
            restitution_coefficient: 0.8,
        }
    }
}

#[derive(Clone)]
pub struct CourseConfig {
    pub one_box_size: f32,
}

impl Default for CourseConfig {
    fn default() -> Self {
        Self { one_box_size: 50.0 }
    }
}
