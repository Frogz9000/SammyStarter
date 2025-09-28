use bevy::prelude::*;
#[derive(Resource)]
pub struct InputMap {
    //movement keys
    pub key_forward: KeyCode,
    pub key_backward: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            key_forward: KeyCode::KeyW,
            key_backward: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
        }
    }
}
