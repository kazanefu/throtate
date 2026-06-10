use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct ScrollUiPlugin;

impl Plugin for ScrollUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scroll_system);
    }
}

#[derive(Component)]
pub struct ScrollCanvas;

fn scroll_sub_canvas_bundle(width: f32, min_height: f32) -> impl Bundle {
    (
        ScrollCanvas,
        Node {
            width: percent(width),
            min_height: percent(min_height),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            margin: UiRect::top(Val::Px(12.0)),
            row_gap: Val::Px(24.0),
            ..default()
        },
    )
}

fn scroll_base_canvas_bundle() -> impl Bundle {
    (Node {
        width: percent(100),
        height: percent(100),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Start,
        flex_direction: FlexDirection::Column,
        overflow: Overflow::clip(),
        padding: UiRect::top(Val::Px(24.0)),
        ..default()
    },)
}

pub struct ScrollUi {
    pub base: Entity,
    pub sub: Entity,
}

pub fn spawn_scroll_canvas(commands: &mut Commands, width: f32, min_height: f32) -> ScrollUi {
    let base = commands.spawn(scroll_base_canvas_bundle()).id();
    let sub = commands
        .spawn(scroll_sub_canvas_bundle(width, min_height))
        .id();
    commands.entity(base).add_child(sub);
    ScrollUi { base, sub }
}

fn scroll_system(
    mut wheel: MessageReader<MouseWheel>,
    mut query: Query<&mut Node, With<ScrollCanvas>>,
    mut offset: Local<f32>,
) {
    for ev in wheel.read() {
        *offset += ev.y * 20.0;
        *offset = offset.clamp(-1000.0, 0.0);

        for mut node in &mut query {
            node.margin.top = Val::Px(12.0 + *offset);
        }
    }
}
