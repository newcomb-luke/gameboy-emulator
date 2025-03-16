use std::{cell::RefCell, rc::Rc};

use audio::Audio;
use lcd::Lcd;
use serial::Serial;

use crate::cpu::bus::Bus;

pub mod audio;
pub mod lcd;
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
    lcd: Lcd,
}

impl IO {
    pub fn new() -> Self {
        Self {
            joypad_input: IORegister::new(),
            serial: Serial::new(),
            audio: Audio::new(),
            lcd: Lcd::new(),
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
    
    pub fn with_lcd_mut<F>(&self, f: F) where F: FnOnce(&mut Lcd) -> () {
        let mut inner = self.inner.borrow_mut();
        f(&mut inner.lcd)
    }
}

impl Bus for SharedIO {
    fn read_u8(&self, address: u16) -> Result<u8, crate::cpu::error::Error> {
        let inner = self.inner.borrow();

        Ok(match address {
            0xFF40 => {
                inner.lcd.read_control()
            }
            0xFF41 => {
                inner.lcd.read_status()
            }
            0xFF42 => {
                inner.lcd.read_scroll_y()
            }
            0xFF43 => {
                inner.lcd.read_scroll_x()
            }
            0xFF44 => inner.lcd.read_lcd_y(),
            0xFF45 => {
                inner.lcd.read_lcd_y_compare()
            }
            0xFF47 => {
                inner.lcd.read_background_palette()
            }
            0xFF48 => {
                inner.lcd.read_obj_palette_0()
            }
            0xFF49 => {
                inner.lcd.read_obj_palette_1()
            }
            0xFF4A => {
                inner.lcd.read_window_y()
            }
            0xFF4B => {
                inner.lcd.read_window_x()
            }
            _ => {
                return Err(crate::cpu::error::Error::MemoryFault(address));
            }
        })
    }

    fn read_u16(&self, address: u16) -> Result<u16, crate::cpu::error::Error> {
        todo!("IO.read_u16(0x{:04x})", address);
    }

    fn write_u8(&self, address: u16, data: u8) -> Result<(), crate::cpu::error::Error> {
        let mut inner = self.inner.borrow_mut();

        match address {
            0xFF10 => {
                inner.audio.channel_1_mut().write_sweep(data);
            }
            0xFF11 => {
                inner
                    .audio
                    .channel_1_mut()
                    .write_length_timer_and_duty_cycle(data);
            }
            0xFF12 => {
                inner.audio.channel_1_mut().write_volume_and_envelope(data);
            }
            0xFF13 => {
                inner.audio.channel_1_mut().write_period_low(data);
            }
            0xFF14 => {
                inner
                    .audio
                    .channel_1_mut()
                    .write_period_high_and_control(data);
            }
            0xFF24 => {
                inner.audio.write_master_volume_vin_panning(data);
            }
            0xFF25 => {
                inner.audio.write_sound_panning(data);
            }
            0xFF26 => {
                inner.audio.write_audio_master_control(data);
            }
            0xFF40 => {
                inner.lcd.write_control(data);
            }
            0xFF41 => {
                inner.lcd.write_status(data);
            }
            0xFF42 => {
                inner.lcd.write_scroll_y(data);
            }
            0xFF43 => {
                inner.lcd.write_scroll_x(data);
            }
            0xFF44 => {
                // Writing is not enabled for LCD Y register
            }
            0xFF45 => {
                inner.lcd.write_lcd_y_compare(data);
            }
            0xFF47 => {
                inner.lcd.write_background_palette(data);
            }
            0xFF48 => {
                inner.lcd.write_obj_palette_0(data);
            }
            0xFF49 => {
                inner.lcd.write_obj_palette_1(data);
            }
            0xFF4A => {
                inner.lcd.write_window_y(data);
            }
            0xFF4B => {
                inner.lcd.write_window_x(data);
            }
            _ => {
                return Err(crate::cpu::error::Error::MemoryFault(address));
            }
        }

        Ok(())
    }

    fn write_u16(&self, address: u16, data: u16) -> Result<(), crate::cpu::error::Error> {
        self.write_u8(address + 1, (data >> 8) as u8)?;
        self.write_u8(address, (data & 0xFF) as u8)
    }
}
