use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

use crate::chaser::MainCameraChaser;
use crate::state::GameState;

pub struct SpaceBackGroundPlugin;

impl Plugin for SpaceBackGroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<SpaceBackGroundMaterial>::default())
            .add_plugins(Material2dPlugin::<LightSpaceBackGroundMaterial>::default());

        app.add_systems(OnEnter(GameState::Playing), spawn_space_background)
            .add_systems(
                Update,
                (
                    update_space_background_material,
                    update_light_space_background_material,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(ShaderType, Clone, Copy, Debug)]
pub struct SpaceBackGroundUniform {
    pub camera_pos: Vec2,
    pub resolution: Vec2,
    pub time: f32,
    pub scale_factor: f32,
}

impl Default for SpaceBackGroundUniform {
    fn default() -> Self {
        Self {
            camera_pos: Vec2::ZERO,
            resolution: Vec2::new(1920.0, 1080.0),
            time: 0.0,
            scale_factor: 1.0,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct SpaceBackGroundMaterial {
    #[uniform(0)]
    pub params: SpaceBackGroundUniform,
}

impl Material2d for SpaceBackGroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/space_background.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct LightSpaceBackGroundMaterial {
    #[uniform(0)]
    pub params: SpaceBackGroundUniform,
}

impl Material2d for LightSpaceBackGroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/light_space_background.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

#[derive(Component)]
pub struct SpaceBackground;

pub fn spawn_space_background(
    mut commands: Commands,
    settings: Res<crate::settings::Settings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SpaceBackGroundMaterial>>,
    mut light_materials: ResMut<Assets<LightSpaceBackGroundMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(10000.0, 10000.0));
    if settings.graphics.light_background {
        commands.spawn((
            DespawnOnExit(GameState::Playing),
            Mesh2d(mesh),
            MeshMaterial2d(light_materials.add(LightSpaceBackGroundMaterial::default())),
            Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
            SpaceBackground,
            MainCameraChaser,
        ));
    } else {
        commands.spawn((
            DespawnOnExit(GameState::Playing),
            Mesh2d(mesh),
            MeshMaterial2d(materials.add(SpaceBackGroundMaterial::default())),
            Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
            SpaceBackground,
            MainCameraChaser,
        ));
    }
}

fn update_space_background_material(
    time: Res<Time>,
    windows: Query<&Window>,
    camera_query: Query<&Transform, (With<Camera>, Without<SpaceBackground>)>,
    mut materials: ResMut<Assets<SpaceBackGroundMaterial>>,
    query: Query<&MeshMaterial2d<SpaceBackGroundMaterial>, With<SpaceBackground>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    // Prevent zero division if window is minimized or not initialized
    if window.width() <= 0.0 || window.height() <= 0.0 {
        return;
    }

    for handle in query.iter() {
        if let Some(material) = materials.get_mut(handle) {
            material.params.camera_pos = camera_transform.translation.truncate();

            material.params.resolution = Vec2::new(
                window.physical_width() as f32,
                window.physical_height() as f32,
            );

            material.params.time = time.elapsed_secs();
            material.params.scale_factor = window.physical_width() as f32 / window.width();
        }
    }
}

fn update_light_space_background_material(
    time: Res<Time>,
    windows: Query<&Window>,
    camera_query: Query<&Transform, (With<Camera>, Without<SpaceBackground>)>,
    mut materials: ResMut<Assets<LightSpaceBackGroundMaterial>>,
    query: Query<&MeshMaterial2d<LightSpaceBackGroundMaterial>, With<SpaceBackground>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    // Prevent zero division if window is minimized or not initialized
    if window.width() <= 0.0 || window.height() <= 0.0 {
        return;
    }

    for handle in query.iter() {
        if let Some(material) = materials.get_mut(handle) {
            material.params.camera_pos = camera_transform.translation.truncate();

            material.params.resolution = Vec2::new(
                window.physical_width() as f32,
                window.physical_height() as f32,
            );

            material.params.time = time.elapsed_secs();
            material.params.scale_factor = window.physical_width() as f32 / window.width();
        }
    }
}
