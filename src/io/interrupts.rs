use super::IORegister;

#[derive(Clone, Copy)]
pub struct Interrupts {
    interrupt_flag: IORegister,
    interrupt_enable: IORegister,
}

impl Interrupts {
    pub fn new() -> Self {
        Self {
            interrupt_flag: IORegister::new(),
            interrupt_enable: IORegister::new(),
        }
    }

    pub fn read_interrupt_enable(&self) -> u8 {
        unimplemented!();
        self.interrupt_enable.read() & 0b0001_1111
    }

    pub fn write_interrupt_enable(&mut self, value: u8) {
        unimplemented!();
        self.interrupt_enable.write(value & 0b0001_1111);
    }

    pub fn read_interrupt_flag(&self) -> u8 {
        unimplemented!();
        self.interrupt_flag.read() & 0b0001_1111
    }

    pub fn write_interrupt_flag(&mut self, value: u8) {
        unimplemented!();
        self.interrupt_flag.write(value & 0b0001_1111);
    }
}
