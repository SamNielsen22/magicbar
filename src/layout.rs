use evdev::KeyCode;

pub struct Key {
    pub label: String,
    pub key: KeyCode,
}

// Keys in a group sit close together; groups have a wider gap between them.
pub struct Layout {
    pub groups: Vec<Vec<Key>>,
}

impl Layout {
    // Apple's own row: esc, screen brightness, keyboard light, media, volume.
    pub fn default_row() -> Self {
        let key = |label: &str, key| Key { label: label.to_string(), key };
        Layout {
            groups: vec![
                vec![key("esc", KeyCode::KEY_ESC)],
                vec![
                    key("󰃞", KeyCode::KEY_BRIGHTNESSDOWN),
                    key("󰃠", KeyCode::KEY_BRIGHTNESSUP),
                ],
                vec![
                    key("󰌌 -", KeyCode::KEY_KBDILLUMDOWN),
                    key("󰌌 +", KeyCode::KEY_KBDILLUMUP),
                ],
                vec![
                    key("󰒮", KeyCode::KEY_PREVIOUSSONG),
                    key("󰐊", KeyCode::KEY_PLAYPAUSE),
                    key("󰒭", KeyCode::KEY_NEXTSONG),
                ],
                vec![
                    key("󰝟", KeyCode::KEY_MUTE),
                    key("󰖀", KeyCode::KEY_VOLUMEDOWN),
                    key("󰕾", KeyCode::KEY_VOLUMEUP),
                ],
            ],
        }
    }

    // All keys left to right, ignoring groups.
    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.groups.iter().flatten()
    }
}
