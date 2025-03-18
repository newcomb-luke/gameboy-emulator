use audio::Audio;
use interrupts::Interrupts;
use lcd::Lcd;
use serial::Serial;

pub mod audio;
pub mod interrupts;
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
    interrupts: Interrupts,
    boot_rom_enable: IORegister,
}

impl IO {
    pub fn new() -> Self {
        Self {
            joypad_input: IORegister::new(),
            serial: Serial::new(),
            audio: Audio::new(),
            lcd: Lcd::new(),
            interrupts: Interrupts::new(),
            boot_rom_enable: IORegister::new(),
        }
    }

    pub fn boot_rom_enable(&self) -> u8 {
        self.boot_rom_enable.0
    }

    pub fn lcd(&self) -> &Lcd {
        &self.lcd
    }

    pub fn lcd_mut(&mut self) -> &mut Lcd {
        &mut self.lcd
    }

    pub fn read_u8(&self, address: u16) -> Result<u8, crate::cpu::error::Error> {
        Ok(match address {
            0xFF40 => self.lcd.read_control(),
            0xFF41 => self.lcd.read_status(),
            0xFF42 => self.lcd.read_scroll_y(),
            0xFF43 => self.lcd.read_scroll_x(),
            0xFF44 => self.lcd.read_lcd_y(),
            0xFF45 => self.lcd.read_lcd_y_compare(),
            0xFF47 => self.lcd.read_background_palette(),
            0xFF48 => self.lcd.read_obj_palette_0(),
            0xFF49 => self.lcd.read_obj_palette_1(),
            0xFF4A => self.lcd.read_window_y(),
            0xFF4B => self.lcd.read_window_x(),
            0xFF50 => self.boot_rom_enable.read(),
            0xFF0F => self.interrupts.read_interrupt_flag(),
            0xFFFF => self.interrupts.read_interrupt_enable(),
            _ => {
                return Err(crate::cpu::error::Error::MemoryFault(address));
            }
        })
    }

    pub fn write_u8(&mut self, address: u16, data: u8) -> Result<(), crate::cpu::error::Error> {
        match address {
            0xFF10 => self.audio.channel_1_mut().write_sweep(data),
            0xFF11 => 
                self.audio
                .channel_1_mut()
                .write_length_timer_and_duty_cycle(data),
            0xFF12 => self.audio.channel_1_mut().write_volume_and_envelope(data),
            0xFF13 => self.audio.channel_1_mut().write_period_low(data),
            0xFF14 => self
                .audio
                .channel_1_mut()
                .write_period_high_and_control(data),
            0xFF24 => self.audio.write_master_volume_vin_panning(data),
            0xFF25 => self.audio.write_sound_panning(data),
            0xFF26 => self.audio.write_audio_master_control(data),
            0xFF40 => self.lcd.write_control(data),
            0xFF41 => self.lcd.write_status(data),
            0xFF42 => self.lcd.write_scroll_y(data),
            0xFF43 => self.lcd.write_scroll_x(data),
            0xFF44 => {} // Writing is not enabled for LCD Y register
            0xFF45 => self.lcd.write_lcd_y_compare(data),
            0xFF47 => self.lcd.write_background_palette(data),
            0xFF48 => self.lcd.write_obj_palette_0(data),
            0xFF49 => self.lcd.write_obj_palette_1(data),
            0xFF4A => self.lcd.write_window_y(data),
            0xFF4B => self.lcd.write_window_x(data),
            0xFF50 => self.boot_rom_enable.write(data),
            0xFF0F => self.interrupts.write_interrupt_flag(data),
            0xFFFF => self.interrupts.write_interrupt_enable(data),
            _ => {
                return Err(crate::cpu::error::Error::MemoryFault(address));
            }
        }

        Ok(())
    }
}