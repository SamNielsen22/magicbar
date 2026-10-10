use std::sync::mpsc;

use crate::actions::Keyboard;
use crate::input::{self, Touch};
use crate::layout::{Key, Layout};
use crate::render::{keys, Canvas};
use crate::transport::drm::DrmDisplay;
use crate::transport::Display;

pub fn run() -> anyhow::Result<()> {
    let layout = Layout::default_row();
    let keys: Vec<&Key> = layout.keys().collect();
    let spans = keys::spans(&layout);
    let mut keyboard = Keyboard::new()?;
    let mut display = DrmDisplay::open()?;
    let (width, _) = display.size();

    let (tx, rx) = mpsc::channel();
    input::spawn_reader(input::find_touch_device()?, tx)?;

    let mut pressed = None;
    draw(&mut display, &layout, pressed)?;
    loop {
        let next = match rx.recv()? {
            Touch::At(x) => keys::key_at(&spans, x * width as f64),
            Touch::Up => {
                if let Some(i) = pressed {
                    tracing::debug!("tap {}", keys[i].label);
                    keyboard.tap(keys[i].key)?;
                }
                None
            }
        };
        if next != pressed {
            pressed = next;
            draw(&mut display, &layout, pressed)?;
        }
    }
}

fn draw(display: &mut DrmDisplay, layout: &Layout, pressed: Option<usize>) -> anyhow::Result<()> {
    let (width, height) = display.size();
    let mut canvas = Canvas::new(width, height)?;
    keys::draw_row(&mut canvas, layout, pressed)?;
    display.present(&canvas)
}
