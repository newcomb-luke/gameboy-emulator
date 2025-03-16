#[derive(Clone, Copy)]
pub struct HighRam {
    contents: [u8; 127],
}

impl HighRam {
    pub fn new() -> Self {
        Self {
            contents: [0u8; 127],
        }
    }

    pub fn contents(&self) -> &[u8; 127] {
        &self.contents
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.contents[(address - 0xFF80) as usize]
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        self.contents[(address - 0xFF80) as usize] = data;
    }
}

#[derive(Clone, Copy)]
pub struct WorkRam {
    contents: [u8; 8192],
}

impl WorkRam {
    pub fn new() -> Self {
        Self {
            contents: [0u8; 8192],
        }
    }

    pub fn contents(&self) -> &[u8; 8192] {
        &self.contents
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.contents[(address - 0xC000) as usize]
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        self.contents[(address - 0xC000) as usize] = data;
    }
}
