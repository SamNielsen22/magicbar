pub mod keys;

use std::f64::consts::FRAC_PI_2;
use std::fs::File;
use std::path::Path;

use anyhow::Context as _;
use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};

pub const FONT: &str = "JetBrainsMono Nerd Font";

pub type Rgb = (f64, f64, f64);

pub struct Canvas {
    surface: ImageSurface,
    width: u32,
    height: u32,
}

impl Canvas {
    // The panel is mounted sideways, so the pixels are stored rotated:
    // a 2170x60 canvas lives in a 60x2170 surface.
    pub fn new(width: u32, height: u32) -> anyhow::Result<Self> {
        let surface = ImageSurface::create(Format::Rgb24, height as i32, width as i32)
            .context("creating cairo surface")?;
        Ok(Self { surface, width, height })
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn draw<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&Context) -> anyhow::Result<()>,
    {
        let cr = Context::new(&self.surface).context("creating cairo context")?;
        cr.translate(self.height as f64, 0.0);
        cr.rotate(FRAC_PI_2);
        f(&cr)
    }

    pub fn fill(&mut self, (r, g, b): Rgb) -> anyhow::Result<()> {
        self.draw(|cr| {
            cr.set_source_rgb(r, g, b);
            cr.paint()?;
            Ok(())
        })
    }

    pub fn text(&mut self, s: &str, size: f64, (r, g, b): Rgb) -> anyhow::Result<()> {
        let mid = self.height as f64 / 2.0;
        self.draw(|cr| {
            cr.select_font_face(FONT, FontSlant::Normal, FontWeight::Normal);
            cr.set_font_size(size);
            cr.set_source_rgb(r, g, b);
            let ext = cr.text_extents(s)?;
            cr.move_to(16.0, mid - ext.y_bearing() - ext.height() / 2.0);
            cr.show_text(s)?;
            Ok(())
        })
    }

    // Raw pixels as stored, ready to copy to the hardware.
    pub fn with_pixels<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&[u8], usize),
    {
        let stride = self.surface.stride() as usize;
        self.surface
            .with_data(|data| f(data, stride))
            .map_err(|e| anyhow::anyhow!("reading canvas pixels: {e}"))
    }

    pub fn save_png(&self, path: &Path) -> anyhow::Result<()> {
        let out = ImageSurface::create(Format::Rgb24, self.width as i32, self.height as i32)?;
        let cr = Context::new(&out)?;
        // Rotate back so the file is landscape like the real bar.
        cr.translate(0.0, self.height as f64);
        cr.rotate(-FRAC_PI_2);
        cr.set_source_surface(&self.surface, 0.0, 0.0)?;
        cr.paint()?;
        drop(cr);

        let mut file = File::create(path).with_context(|| format!("creating {}", path.display()))?;
        out.write_to_png(&mut file)?;
        Ok(())
    }
}
