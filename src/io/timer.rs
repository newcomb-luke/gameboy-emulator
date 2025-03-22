use super::IORegister;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSelect {
    Every256MCycles,
    Every4MCycles,
    Every16MCycles,
    Every64MCycles,
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
}

impl Timer {
    pub fn new() -> Self {
        Self {
            divider: IORegister::new(),
            timer_counter: IORegister::new(),
            timer_modulo: IORegister::new(),
            timer_control: TimerControl::new(),
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
        if self.timer_control.enable {
            todo!()
        }

        false
    }
}
