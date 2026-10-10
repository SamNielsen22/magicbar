use std::time::Duration;

use anyhow::Context;
use evdev::uinput::VirtualDevice;
use evdev::{AttributeSet, KeyCode, KeyEvent};

pub struct Keyboard(VirtualDevice);

impl Keyboard {
    pub fn new() -> anyhow::Result<Self> {
        // Register every key so any layout works. 0x2ff is KEY_MAX in the kernel.
        let keys: AttributeSet<KeyCode> = (0..=0x2ff).map(KeyCode).collect();
        let device = VirtualDevice::builder()?
            .name("magicbar")
            .with_keys(&keys)?
            .build()
            .context("creating virtual keyboard")?;
        // Give the compositor a moment to notice the new keyboard.
        std::thread::sleep(Duration::from_millis(200));
        Ok(Keyboard(device))
    }

    pub fn tap(&mut self, key: KeyCode) -> anyhow::Result<()> {
        self.0.emit(&[*KeyEvent::new(key, 1)])?;
        self.0.emit(&[*KeyEvent::new(key, 0)])?;
        Ok(())
    }
}
