use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        let config: InputConfig = InputConfig {
            scan_codes: [
                [Some(ScanCode(17)), None], // Up
                [Some(ScanCode(31)), None], // Down
                [Some(ScanCode(30)), None], // Left
                [Some(ScanCode(32)), None], // Right
                [Some(ScanCode(57)), None], // Dash
                [None, None],               // Shoot
                [None, None],               // Melee
                [Some(ScanCode(33)), None], // Energy to health
            ],
            mouse_buttons: [
                None,                     // Up
                None,                     // Down
                None,                     // Left
                None,                     // Right
                None,                     // Dash
                Some(MouseButton::Left),  // Shoot
                Some(MouseButton::Right), // Melee
                None,                     // Energy to health
            ],
        };

        app.insert_resource(config);
    }
}

const MAP_SIZE: usize = 8;
pub enum InputMap {
    Up = 0,
    Down,
    Left,
    Right,
    Dash,
    Shoot,
    Melee,
    EnergyToHealth,
}

#[derive(Resource)]
pub struct InputConfig {
    scan_codes: [[Option<ScanCode>; 2]; MAP_SIZE],
    mouse_buttons: [Option<MouseButton>; MAP_SIZE],
}

impl InputConfig {
    pub fn get_scan_codes(&self, action: InputMap) -> Vec<ScanCode> {
        self.scan_codes[action as usize].into_iter().flatten().collect()
    }

    pub fn get_mouse_buttons(&self, action: InputMap) -> Option<MouseButton> {
        self.mouse_buttons[action as usize]
    }
}
