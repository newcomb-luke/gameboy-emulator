use super::IORegister;

#[derive(Debug, Clone, Copy)]
pub struct Serial {
    data: IORegister,
    control: IORegister,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            data: IORegister::new(),
            control: IORegister::new(),
        }
    }

    pub fn write_data(&mut self, value: u8) {
        self.data.write(value);
    }

    pub fn read_data(&self) -> u8 {
        self.data.read()
    }

    pub fn write_control(&mut self, value: u8) {
        self.control.write(value);
    }

    pub fn read_control(&self) -> u8 {
        self.control.read()
    }
}
