use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use audio::Audio;
use serial::Serial;

use crate::cpu::bus::Bus;

pub mod audio;
pub mod serial;

#[derive(Debug, Clone, Copy)]
pub struct IORegister(u8);

impl IORegister {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn write(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn read(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct IO {
    joypad_input: IORegister,
    serial: Serial,
    audio: Audio,
}

impl IO {
    pub fn new() -> Self {
        Self {
            joypad_input: IORegister::new(),
            serial: Serial::new(),
            audio: Audio::new(),
        }
    }
}

#[derive(Clone)]
pub struct SharedIO {
    inner: Rc<RefCell<IO>>,
}

impl SharedIO {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(IO::new())),
        }
    }
}

impl Bus for SharedIO {
    fn read_u8(&self, address: u16) -> Result<u8, crate::cpu::error::Error> {
        todo!()
    }

    fn read_u16(&self, address: u16) -> Result<u16, crate::cpu::error::Error> {
        todo!()
    }

    fn write_u8(&self, address: u16, data: u8) -> Result<(), crate::cpu::error::Error> {
        let mut inner = self.inner.borrow_mut();

        match address {
            0xFF26 => {
                inner.audio.write_audio_master_control(data);
            }
            0xFF25 => {
                inner.audio.write_sound_panning(data);
            }
            0xFF24 => {
                inner.audio.write_master_volume_vin_panning(data);
            }
            _ => {
                return Err(crate::cpu::error::Error::MemoryFault);
            }
        }

        Ok(())
    }

    fn write_u16(&self, address: u16, data: u16) -> Result<(), crate::cpu::error::Error> {
        self.write_u8(address + 1, (data >> 8) as u8)?;
        self.write_u8(address, (data & 0xFF) as u8)
    }
}
