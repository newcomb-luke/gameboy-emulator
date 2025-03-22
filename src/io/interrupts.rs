use super::IORegister;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {
    Joypad,
    Serial,
    Timer,
    Lcd,
    VBlank,
}

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

    pub fn highest_priority_triggered_interrupt(&self) -> Option<Interrupt> {
        let triggered = self.interrupt_enable.read() & self.interrupt_flag.read();

        if (triggered & 0b0000_0001) != 0 {
            Some(Interrupt::VBlank)
        } else if (triggered & 0b0000_0010) != 0 {
            Some(Interrupt::Lcd)
        } else if (triggered & 0b0000_0100) != 0 {
            Some(Interrupt::Timer)
        } else if (triggered & 0b0000_1000) != 0 {
            Some(Interrupt::Serial)
        } else if (triggered & 0b0001_0000) != 0 {
            Some(Interrupt::Joypad)
        } else {
            None
        }
    }

    pub fn clear_requested_interrupt(&mut self, interrupt: Interrupt) {
        let mask = !match interrupt {
            Interrupt::VBlank => 1 << 0,
            Interrupt::Lcd => 1 << 1,
            Interrupt::Timer => 1 << 2,
            Interrupt::Serial => 1 << 3,
            Interrupt::Joypad => 1 << 4,
        };

        self.interrupt_flag.write(self.interrupt_flag.read() & mask);
    }

    pub fn set_interrupt_requested(&mut self, interrupt: Interrupt) {
        let bit = match interrupt {
            Interrupt::VBlank => 1 << 0,
            Interrupt::Lcd => 1 << 1,
            Interrupt::Timer => 1 << 2,
            Interrupt::Serial => 1 << 3,
            Interrupt::Joypad => 1 << 4,
        };

        self.interrupt_flag.write(self.interrupt_flag.read() | bit);
    }

    pub fn read_interrupt_enable(&self) -> u8 {
        self.interrupt_enable.read() & 0b0001_1111
    }

    pub fn write_interrupt_enable(&mut self, value: u8) {
        self.interrupt_enable.write(value & 0b0001_1111);
    }

    pub fn read_interrupt_flag(&self) -> u8 {
        self.interrupt_flag.read() & 0b0001_1111
    }

    pub fn write_interrupt_flag(&mut self, value: u8) {
        self.interrupt_flag.write(value & 0b0001_1111);
    }
}
