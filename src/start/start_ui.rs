use crate::state::GameState;
use crate::utils::FONT_PATH;
use bevy::prelude::*;
const EXPLANATION_TEXT: &str = r#"
概要: 
    ハンマー投のようにぐるぐる回してから離すことで移動してゴールを目指すゲームです。
登場するもの:
    プレイヤー:
        青い立方体の見た目で操作可能
    赤色の立方体:
        当たると死ぬ
    水色の立方体:
        チェックポイント
    オレンジ色の立方体:
        小さな赤色の立方体を発射する
    黄色の立方体:
        一定速度以上の物体が当たると壊れる
    マゼンタの立方体:
        ゴール
    黄土色:
        地形
操作方法:
    Spaceキーで拘束して回転と拘束を解くのを切り替える
    矢印キーで回転軸の相対座標と回転方向を切り替える
    Rキーでチェックポイントに戻る
"#;

pub struct StartUiPlugin;

impl Plugin for StartUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), spawn_start_ui)
            .add_systems(
                Update,
                (update_start_button, scroll_system).run_if(in_state(GameState::Start)),
            );
    }
}

#[derive(Component)]
struct StartButton;

fn start_button_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Button,
        StartButton,
        Node {
            width: percent(20),
            height: percent(10),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.9, 0.2)),
        children![(
            Text::new("スタート"),
            TextFont {
                font: asset_server.load(FONT_PATH),
                font_size: 40.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgb(0.2, 0.2, 0.2))
        )],
    )
}

fn explanation_text_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new(EXPLANATION_TEXT),
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Left),
        TextColor::WHITE,
    )
}

fn start_canvas_bundle() -> impl Bundle {
    (
        DespawnOnExit(GameState::Start),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip(),
            ..default()
        },
    )
}

fn spawn_start_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let canvas = commands.spawn(start_canvas_bundle()).id();
    let sub_canvas = commands.spawn(start_sub_canvas_bundle()).id();
    let explanation_text = commands.spawn(explanation_text_bundle(&asset_server)).id();
    let start_button = commands.spawn(start_button_bundle(&asset_server)).id();
    commands
        .entity(sub_canvas)
        .add_children(&[explanation_text, start_button]);
    commands.entity(canvas).add_child(sub_canvas);
}

type StartButtonInputs = (Changed<Interaction>, With<StartButton>);

fn update_start_button(
    mut game_state: ResMut<NextState<GameState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor), StartButtonInputs>,
    key: Res<ButtonInput<KeyCode>>,
) {
    for (interaction, mut background_color) in &mut query {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.2, 1.0, 0.3);
                game_state.set(GameState::CourseSelection);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.1, 0.8, 0.4);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.0, 0.5, 0.5);
            }
        }
    }
    if key.just_pressed(KeyCode::Enter) {
        game_state.set(GameState::CourseSelection);
    }
}

use crate::course_selection::selection_ui::ScrollContent;
use bevy::input::mouse::MouseWheel;

fn start_sub_canvas_bundle() -> impl Bundle {
    (
        ScrollContent,
        Node {
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            top: Val::Px(0.0),
            row_gap: Val::Px(10.0),
            ..default()
        }
    )
}

fn scroll_system(
    mut wheel: MessageReader<MouseWheel>,
    mut query: Query<&mut Node, With<ScrollContent>>,
    mut offset: Local<f32>,
) {
    for ev in wheel.read() {
        *offset += ev.y * 20.0;

        *offset = offset.clamp(-1000.0, 300.0);

        for mut node in &mut query {
            node.top = Val::Px(*offset);
        }
    }
}
