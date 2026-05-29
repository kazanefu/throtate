use bevy::{input::mouse::MouseMotion, prelude::*};
use std::collections::HashMap;

pub struct KeyboardButtonPlugin;

impl Plugin for KeyboardButtonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiInputMode>()
            .init_resource::<SelectedButton>()
            .add_systems(
                Update,
                (
                    update_ui_input_mode,
                    add_buttons,
                    remove_buttons,
                    keyboard_ui_navigation,
                )
                    .chain(),
            )
            .add_systems(PostUpdate, apply_keyboard_selection);
    }
}

#[derive(Resource, PartialEq, Eq, Default)]
pub enum UiInputMode {
    #[default]
    Mouse,
    Keyboard,
}

#[derive(Component)]
pub struct KeyboardHovered(pub bool);

#[derive(Resource, Default)]
pub struct SelectedButton {
    pub id: Option<usize>,
    pub buttons: HashMap<usize, Entity>,
}

#[derive(Component)]
pub struct SelectableButton {
    pub id: usize,
}

fn update_ui_input_mode(
    mut mode: ResMut<UiInputMode>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mouse_used = !mouse_motion.is_empty() || mouse_buttons.just_pressed(MouseButton::Left);

    let keyboard_used = keyboard.just_pressed(KeyCode::Tab)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::ArrowDown)
        || keyboard.just_pressed(KeyCode::ArrowLeft)
        || keyboard.just_pressed(KeyCode::ArrowRight);

    if mouse_used {
        *mode = UiInputMode::Mouse;
    }

    if keyboard_used {
        *mode = UiInputMode::Keyboard;
    }

    mouse_motion.clear();
}

fn add_buttons(
    added: Query<(Entity, &SelectableButton), Added<SelectableButton>>,
    mut selected: ResMut<SelectedButton>,
) {
    for (entity, button) in &added {
        selected.buttons.insert(button.id, entity);
    }
}

fn remove_buttons(
    mut removed: RemovedComponents<SelectableButton>,
    mut selected: ResMut<SelectedButton>,
) {
    for entity in removed.read() {
        let remove_id = selected
            .buttons
            .iter()
            .find_map(|(&id, &e)| (e == entity).then_some(id));

        if let Some(id) = remove_id {
            selected.buttons.remove(&id);

            if selected.id == Some(id) {
                selected.id = None;
            }
        }
    }
}

fn keyboard_ui_navigation(
    mode: Res<UiInputMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedButton>,
) {
    if *mode != UiInputMode::Keyboard {
        selected.id = None;
        return;
    }

    if selected.buttons.is_empty() {
        selected.id = None;
        return;
    }

    let mut ids: Vec<_> = selected.buttons.keys().copied().collect();

    ids.sort_unstable();

    let current_index = selected.id.and_then(|id| ids.iter().position(|&x| x == id));

    let next = keyboard.just_pressed(KeyCode::Tab)
        || keyboard.just_pressed(KeyCode::ArrowRight)
        || keyboard.just_pressed(KeyCode::ArrowDown);

    let prev = ((keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight))
        && keyboard.just_pressed(KeyCode::Tab))
        || keyboard.just_pressed(KeyCode::ArrowLeft)
        || keyboard.just_pressed(KeyCode::ArrowUp);

    if next {
        let next_index = current_index.map(|i| (i + 1) % ids.len()).unwrap_or(0);

        selected.id = Some(ids[next_index]);
    }

    if prev {
        let prev_index = current_index
            .map(|i| (i + ids.len() - 1) % ids.len())
            .unwrap_or(ids.len() - 1);

        selected.id = Some(ids[prev_index]);
    }
}

fn apply_keyboard_selection(
    mut commands: Commands,
    mode: Res<UiInputMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
    selection: Res<SelectedButton>,
    mut query: Query<
        (
            Entity,
            &SelectableButton,
            &mut Interaction,
            Option<&KeyboardHovered>,
        ),
        With<Button>,
    >,
) {
    if *mode != UiInputMode::Keyboard {
        for (entity, _, mut interaction, _) in &mut query {
            *interaction = Interaction::None;
            commands.entity(entity).remove::<KeyboardHovered>();
        }
        return;
    }

    for (entity, button, mut interaction, keyboad_hovered) in &mut query {
        if Some(button.id) == selection.id {
            if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
                *interaction = Interaction::Pressed;
            } else {
                *interaction = Interaction::Hovered;
                if keyboad_hovered.is_none() {
                    commands.entity(entity).insert(KeyboardHovered(true));
                }
            }
        } else {
            *interaction = Interaction::None;
            commands.entity(entity).remove::<KeyboardHovered>();
        }
    }
}
