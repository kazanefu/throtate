use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct KeyBindings {
    keys: [Option<KeyCode>; Self::MAX_KEYS],
}

impl KeyBindings {
    pub const MAX_KEYS: usize = 4;

    pub const fn new(keys: [Option<KeyCode>; Self::MAX_KEYS]) -> Self {
        Self { keys }
    }

    pub fn just_pressed(&self, input: &ButtonInput<KeyCode>) -> bool {
        self.keys
            .iter()
            .flatten()
            .any(|&key| input.just_pressed(key))
    }
}

#[derive(Clone, Copy)]
pub struct InputSetting {
    pub respawn: KeyBindings,
    pub throw: KeyBindings,
    pub ll_spin: KeyBindings,
    pub lr_spin: KeyBindings,
    pub rl_spin: KeyBindings,
    pub rr_spin: KeyBindings,
    pub next: KeyBindings,
    pub exit: KeyBindings,
}

impl Default for InputSetting {
    fn default() -> Self {
        Self {
            respawn: KeyBindings::new([Some(KeyCode::KeyR), None, None, None]),
            throw: KeyBindings::new([Some(KeyCode::Space), None, None, None]),
            ll_spin: KeyBindings::new([
                Some(KeyCode::ArrowLeft),
                Some(KeyCode::KeyA),
                Some(KeyCode::KeyN),
                None,
            ]),
            lr_spin: KeyBindings::new([
                Some(KeyCode::ArrowDown),
                Some(KeyCode::KeyS),
                Some(KeyCode::KeyE),
                None,
            ]),
            rl_spin: KeyBindings::new([
                Some(KeyCode::ArrowUp),
                Some(KeyCode::KeyW),
                Some(KeyCode::KeyU),
                None,
            ]),
            rr_spin: KeyBindings::new([
                Some(KeyCode::ArrowRight),
                Some(KeyCode::KeyD),
                Some(KeyCode::KeyI),
                None,
            ]),
            next: KeyBindings::new([Some(KeyCode::Enter), Some(KeyCode::Space), None, None]),
            exit: KeyBindings::new([Some(KeyCode::Escape), None, None, None]),
        }
    }
}
