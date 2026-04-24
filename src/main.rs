use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod course;
mod course_selection;
mod hammer;
mod start;
mod state;
mod utils;
mod playing;

pub use utils::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(utils::UtilityPlugin)
        .init_state::<state::GameState>()
        .add_plugins(course::CoursePlugin)
        .add_plugins(hammer::HammerPlugin)
        .add_plugins(start::StartPlugin)
        .add_plugins(playing::PlayingPlugin)
        .add_plugins(course_selection::CourseSelectionPlugin)
        .run();
}

#[allow(unused)]
fn setup(mut commands: Commands) {
    // カメラ
    commands.spawn(Camera2d);

    // 床
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(200.0, 10.0),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.2),
            custom_size: Some(Vec2::new(400.0, 20.0)),
            ..default()
        },
    ));

    hammer::spawn_hammer(&mut commands, Vec2 { x: 0.0, y: 0.0 });
}
