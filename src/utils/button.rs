use super::*;
use bevy::prelude::*;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_button_target_scale, linear_interpolation_scale).in_set(UtilitySystemSet),
        );
    }
}

#[derive(Component)]
pub struct SizeUpButton {
    rate: f32,
}

impl SizeUpButton {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

#[derive(Component)]
pub struct UiScaleLinearInterpolation {
    target: Vec2,
    speed: f32,
}

impl UiScaleLinearInterpolation {
    pub fn new(target: Vec2, speed: f32) -> Self {
        Self { target, speed }
    }

    pub fn from_speed(speed: f32) -> Self {
        Self::new(Vec2::ONE, speed)
    }
}

use crate::{audio::button::ButtonSounds, keyboard_button::KeyboardHovered};

#[derive(Bundle)]
pub struct SizeUpButtonBundle {
    sizeup_button: SizeUpButton,
    interpolation: UiScaleLinearInterpolation,
    button_sounds: ButtonSounds,
}

impl SizeUpButtonBundle {
    pub fn new(rate: f32, speed: f32) -> Self {
        Self {
            sizeup_button: SizeUpButton::new(rate),
            interpolation: UiScaleLinearInterpolation::from_speed(speed),
            button_sounds: ButtonSounds,
        }
    }
}

type SizeUpButtonInputs = (Changed<Interaction>, With<SizeUpButton>);

fn update_button_target_scale(
    mut query: Query<
        (
            &Interaction,
            &mut UiScaleLinearInterpolation,
            &SizeUpButton,
            Option<&KeyboardHovered>,
        ),
        SizeUpButtonInputs,
    >,
) {
    for (interaction, mut interpolation, sizeup, keyboard_hovered) in &mut query {
        interpolation.target = match *interaction {
            Interaction::Hovered => Vec2::splat(sizeup.rate),
            _ => Vec2::ONE,
        };
        if keyboard_hovered.is_some() {
            interpolation.target = Vec2::splat(sizeup.rate);
        }
    }
}

fn linear_interpolation_scale(
    time: Res<Time>,
    mut query: Query<(&mut UiTransform, &UiScaleLinearInterpolation)>,
) {
    for (mut ui_transform, interpolation) in &mut query {
        ui_transform.scale = ui_transform.scale.lerp(
            interpolation.target,
            interpolation.speed * time.delta_secs(),
        );
    }
}
