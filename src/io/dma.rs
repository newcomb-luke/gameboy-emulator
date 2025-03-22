use super::IORegister;

pub const DMA_TRANSFER_CYCLES_LENGTH: u16 = 160;

#[derive(Debug, Clone, Copy)]
pub struct DMAController {
    transferring: bool,
    source_address: u16,
    cycles_in: u16,
    source_reg: IORegister,
}

impl DMAController {
    pub fn new() -> Self {
        Self {
            transferring: false,
            source_address: 0,
            cycles_in: 0,
            source_reg: IORegister::new(),
        }
    }

    pub fn read_source_address(&self) -> u8 {
        self.source_reg.read()
    }

    pub fn full_source_address(&self) -> u16 {
        self.source_address
    }

    pub fn start_new_transfer(&mut self, source: u8) {
        self.transferring = true;
        self.source_address = source as u16 * 0x100;
        self.source_reg.write(source);
        self.cycles_in = 0;
    }

    pub fn transferring(&self) -> bool {
        self.transferring
    }

    pub fn step(&mut self, cycles: usize) -> bool {
        if self.transferring {
            let now = self.cycles_in + cycles as u16;

            if now > DMA_TRANSFER_CYCLES_LENGTH {
                self.cycles_in = 0;
                self.transferring = false;
                return true;
            } else {
                self.cycles_in += now;
            }
        }

        false
    }
}
