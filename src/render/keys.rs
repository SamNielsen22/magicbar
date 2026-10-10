use std::f64::consts::{FRAC_PI_2, PI};

use cairo::{Context, FontSlant, FontWeight};

use super::{Canvas, Rgb, FONT};
use crate::layout::Layout;


const MARGIN: f64 = 32.0;
const KEY_WIDTH: f64 = 160.0;
const GAP: f64 = 4.0;
const GROUP_GAP: f64 = 64.0;
const ICON_SIZE: f64 = 40.0;
const TEXT_SIZE: f64 = 26.0;
const INSET: f64 = 5.0;
const RADIUS: f64 = 10.0;
const PLATE: Rgb = (0.25, 0.25, 0.25);
const PLATE_PRESSED: Rgb = (0.45, 0.45, 0.45);
const LABEL: Rgb = (1.0, 1.0, 1.0);

// Each key's left and right edge in pixels, in layout order.
pub type Spans = Vec<(f64, f64)>;

pub fn spans(layout: &Layout) -> Spans {
    let mut spans = Vec::new();
    let mut x = MARGIN;
    for group in &layout.groups {
        for _ in group {
            spans.push((x, x + KEY_WIDTH));
            x += KEY_WIDTH + GAP;
        }
        x += GROUP_GAP - GAP;
    }
    spans
}

pub fn draw_row(canvas: &mut Canvas, layout: &Layout, pressed: Option<usize>) -> anyhow::Result<()> {
    let (_, height) = canvas.size();
    let height = height as f64;
    let spans = spans(layout);

    canvas.fill((0.0, 0.0, 0.0))?;
    canvas.draw(|cr| {
        cr.select_font_face(FONT, FontSlant::Normal, FontWeight::Normal);
        for (i, key) in layout.keys().enumerate() {
            let size = if key.label.is_ascii() { TEXT_SIZE } else { ICON_SIZE };
            cr.set_font_size(size);
            let (left, right) = spans[i];
            let (r, g, b) = if pressed == Some(i) { PLATE_PRESSED } else { PLATE };
            cr.set_source_rgb(r, g, b);
            rounded_rect(cr, left, INSET, right - left, height - 2.0 * INSET);
            cr.fill()?;

            let (r, g, b) = LABEL;
            cr.set_source_rgb(r, g, b);
            let ext = cr.text_extents(&key.label)?;
            cr.move_to(
                (left + right) / 2.0 - ext.width() / 2.0 - ext.x_bearing(),
                height / 2.0 - ext.height() / 2.0 - ext.y_bearing(),
            );
            cr.show_text(&key.label)?;
        }
        Ok(())
    })
}

pub fn key_at(spans: &Spans, x: f64) -> Option<usize> {
    spans.iter().position(|&(left, right)| x >= left && x < right)
}

fn rounded_rect(cr: &Context, x: f64, y: f64, w: f64, h: f64) {
    cr.new_sub_path();
    cr.arc(x + w - RADIUS, y + RADIUS, RADIUS, -FRAC_PI_2, 0.0);
    cr.arc(x + w - RADIUS, y + h - RADIUS, RADIUS, 0.0, FRAC_PI_2);
    cr.arc(x + RADIUS, y + h - RADIUS, RADIUS, FRAC_PI_2, PI);
    cr.arc(x + RADIUS, y + RADIUS, RADIUS, PI, 3.0 * FRAC_PI_2);
    cr.close_path();
}
