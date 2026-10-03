pub mod drm;

use crate::render::Canvas;

pub trait Display {
    /// Size as you look at it, (2170, 60) on the T1.
    fn size(&self) -> (u32, u32);
    fn present(&mut self, canvas: &Canvas) -> anyhow::Result<()>;
}
