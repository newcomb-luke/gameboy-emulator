use vram::Vram;

pub mod vram;

pub struct Ppu {
    vram: Vram
}

impl Ppu {
    pub fn new(vram: Vram) -> Self {
        Self {
            vram
        }
    }
}