use crate::button::SizeUpButtonBundle;
use crate::course::CourseEntry;
use crate::course_selection::resources::SelectedCourseID;
use crate::{course::CourseListResource, state::GameState, *};
use bevy::input::mouse::MouseWheel;
use std::fmt::Write;
pub struct SelectionUiPlugin;

impl Plugin for SelectionUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CourseSelection), spawn_selection_ui)
            .add_systems(
                Update,
                (
                    scroll_system,
                    update_course_list_buttons,
                    update_confirm_button_text,
                    update_confirm_button,
                )
                    .run_if(in_state(GameState::CourseSelection)),
            );
    }
}

const SELECTION_EXPLANATION: &str = r#"
プレイするコースを選択してください
"#;

#[derive(Component)]
struct ConfirmButton;

#[derive(Component)]
struct ConfirmButtonText;

fn confirm_ui_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        Button,
        ConfirmButton,
        SizeUpButtonBundle::new(1.1, 10.0),
        UiTransform::default(),
        Node {
            width: Val::Px(520.0),
            min_width: percent(30),
            min_height: Val::Px(72.0),
            padding: UiRect::axes(Val::Px(24.0), Val::Px(16.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        children![(
            ConfirmButtonText,
            Node {
                max_width: percent(100),
                ..default()
            },
            Text::new("Confirm"),
            TextFont {
                font: font.clone(),
                font_size: 32.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgb(0.0, 0.0, 0.0))
        )],
    )
}

fn selection_explanation_text_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(SELECTION_EXPLANATION),
        TextFont {
            font: font.clone(),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
        TextColor::WHITE,
    )
}

fn selection_canvas_bundle() -> impl Bundle {
    (
        DespawnOnExit(GameState::CourseSelection),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip(),
            padding: UiRect::top(Val::Px(24.0)),
            ..default()
        },
    )
}

#[derive(Component)]
pub struct ScrollContent;

fn selection_sub_canvas_bundle() -> impl Bundle {
    (
        ScrollContent,
        Node {
            width: percent(100),
            min_height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            margin: UiRect::top(Val::Px(12.0)),
            row_gap: Val::Px(24.0),
            ..default()
        },
    )
}

fn course_list_button_node_bundle(_len: usize) -> impl Bundle {
    (Node {
        width: percent(30),
        min_width: Val::Px(320.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(10.0),
        ..default()
    },)
}

#[derive(Component)]
struct CourseListButton(usize);

fn course_list_button_bundle(
    font: &Handle<Font>,
    course_entry: &CourseEntry,
    _len: usize,
) -> impl Bundle {
    (
        Button,
        CourseListButton(course_entry.id),
        SizeUpButtonBundle::new(1.1, 10.0),
        UiTransform::default(),
        Node {
            width: percent(100),
            min_height: Val::Px(64.0),
            padding: UiRect::axes(Val::Px(16.0), Val::Px(12.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.9, 0.2)),
        children![(
            Node {
                max_width: percent(100),
                ..default()
            },
            Text::new(&course_entry.name),
            TextFont {
                font: font.clone(),
                font_size: 30.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgb(0.0, 0.0, 0.0))
        )],
    )
}

fn spawn_selection_ui(
    mut commands: Commands,
    font: Res<JpFont>,
    course_list_res: Res<CourseListResource>,
) {
    let canvas = commands.spawn(selection_canvas_bundle()).id();
    let sub_canvas = commands.spawn(selection_sub_canvas_bundle()).id();
    let explanation_text = commands
        .spawn(selection_explanation_text_bundle(font.get()))
        .id();
    let confirm_button = commands.spawn(confirm_ui_bundle(font.get())).id();
    let course_list_button = commands
        .spawn(course_list_button_node_bundle(course_list_res.0.len()))
        .id();
    let mut course_buttons = Vec::new();
    for (course_entry, _) in &course_list_res.0 {
        course_buttons.push(
            commands
                .spawn(course_list_button_bundle(
                    font.get(),
                    course_entry,
                    course_list_res.0.len(),
                ))
                .id(),
        );
    }
    commands
        .entity(course_list_button)
        .add_children(&course_buttons);
    commands.entity(sub_canvas).add_children(&[
        explanation_text,
        confirm_button,
        course_list_button,
    ]);
    commands.entity(canvas).add_child(sub_canvas);
}

fn scroll_system(
    mut wheel: MessageReader<MouseWheel>,
    mut query: Query<&mut Node, With<ScrollContent>>,
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

fn update_course_list_buttons(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &CourseListButton),
        Changed<Interaction>,
    >,
    mut selected_id: ResMut<SelectedCourseID>,
) {
    for (interaction, mut background_color, button) in &mut button_query {
        match interaction {
            Interaction::Pressed => {
                if selected_id.0 != Some(button.0) {
                    selected_id.0 = Some(button.0);
                }
                background_color.0 = Color::srgb(0.2, 0.2, 0.2);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.0, 0.9, 0.0);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.1, 0.9, 0.2);
            }
        }
    }
}

fn update_confirm_button_text(
    mut text_query: Query<&mut Text, With<ConfirmButtonText>>,
    selected_id: Res<SelectedCourseID>,
    course_list_res: Res<CourseListResource>,
) {
    if !selected_id.is_changed() {
        return;
    }

    let name = if let Some(id) = selected_id.0 {
        &course_list_res
            .0
            .iter()
            .find(|x| x.0.id == id)
            .expect("such id course doesn't exists")
            .0
            .name
    } else {
        "None"
    };
    for mut text in &mut text_query {
        text.0.clear();
        write!(&mut text.0, "決定: {}", name).expect("writing to String should never fail");
    }
}

type ConfirmButtonInputs = (Changed<Interaction>, With<ConfirmButton>);

fn update_confirm_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), ConfirmButtonInputs>,
    selected_id: Res<SelectedCourseID>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if selected_id.0.is_none() {
        return;
    }
    for (interaction, mut background_color) in &mut button_query {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.2, 0.2, 0.5);
                game_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.0, 0.9, 0.9);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.1, 0.9, 0.9);
            }
        }
    }
}
