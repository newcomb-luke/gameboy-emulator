use super::IORegister;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSelect {
    Every256MCycles,
    Every4MCycles,
    Every16MCycles,
    Every64MCycles,
}

impl ClockSelect {
    pub fn cycles_value(&self) -> usize {
        match self {
            Self::Every256MCycles => 256,
            Self::Every4MCycles => 4,
            Self::Every16MCycles => 16,
            Self::Every64MCycles => 64,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TimerControl {
    enable: bool,
    clock_select: ClockSelect,
}

impl TimerControl {
    pub fn new() -> Self {
        Self {
            enable: false,
            clock_select: ClockSelect::Every256MCycles,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Timer {
    divider: IORegister,
    timer_counter: IORegister,
    timer_modulo: IORegister,
    timer_control: TimerControl,
    cycles: usize,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            divider: IORegister::new(),
            timer_counter: IORegister::new(),
            timer_modulo: IORegister::new(),
            timer_control: TimerControl::new(),
            cycles: 0,
        }
    }

    pub fn read_divider(&self) -> u8 {
        self.divider.read()
    }

    pub fn set_divider(&mut self, value: u8) {
        self.divider.write(value);
    }

    pub fn write_divider(&mut self, _value: u8) {
        self.divider.write(0);
    }

    pub fn read_timer_counter(&self) -> u8 {
        self.timer_counter.read()
    }

    pub fn write_timer_counter(&mut self, value: u8) {
        self.timer_counter.write(value);
    }

    pub fn read_timer_modulo(&self) -> u8 {
        self.timer_modulo.read()
    }

    pub fn write_timer_modulo(&mut self, value: u8) {
        self.timer_modulo.write(value);
    }

    pub fn read_timer_control(&self) -> u8 {
        let mut value = if self.timer_control.enable { 1 } else { 0 } << 2;
        value |= match self.timer_control.clock_select {
            ClockSelect::Every256MCycles => 0,
            ClockSelect::Every4MCycles => 1,
            ClockSelect::Every16MCycles => 2,
            ClockSelect::Every64MCycles => 3,
        };
        value
    }

    pub fn write_timer_control(&mut self, value: u8) {
        self.timer_control.enable = (value & 0b0000_0100) != 0;
        self.timer_control.clock_select = match value & 0b0000_0011 {
            0 => ClockSelect::Every256MCycles,
            1 => ClockSelect::Every4MCycles,
            2 => ClockSelect::Every16MCycles,
            _ => ClockSelect::Every64MCycles,
        };
    }

    pub fn step(&mut self, cycles: usize) -> bool {
        self.divider
            .write(self.divider.read().wrapping_add((cycles & 0xFF) as u8));

        if !self.timer_control.enable {
            return false;
        }

        self.cycles += cycles;

        let current_cycles_value = self.timer_control.clock_select.cycles_value();

        let counter_increments = (self.cycles / current_cycles_value) as u8;
        self.cycles = self.cycles % current_cycles_value;

        let (_, overflowed) = self
            .timer_counter
            .read()
            .overflowing_add(counter_increments);

        if overflowed {
            // Reset the timer counter to the value in timer modulo
            self.timer_counter.write(self.timer_modulo.read());
        }

        overflowed
    }
}
