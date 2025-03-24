use audio::Audio;
use dma::DMAController;
use interrupts::Interrupts;
use joypad::JoypadInput;
use lcd::Lcd;
use serial::Serial;
use timer::Timer;

pub mod audio;
pub mod dma;
pub mod interrupts;
pub mod joypad;
pub mod lcd;
pub mod serial;
pub mod timer;

#[derive(Debug, Clone, Copy)]
pub struct IORegister(u8);

impl IORegister {
    pub fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub fn write(&mut self, value: u8) {
        self.0 = value;
    }

    #[inline(always)]
    pub fn read(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct IO {
    joypad_input: JoypadInput,
    lcd: Lcd,
    serial: Serial,
    audio: Audio,
    timer: Timer,
    interrupts: Interrupts,
    dma: DMAController,
    boot_rom_enable: IORegister,
}

impl IO {
    pub fn new() -> Self {
        Self {
            joypad_input: JoypadInput::new(),
            lcd: Lcd::new(),
            serial: Serial::new(),
            audio: Audio::new(),
            timer: Timer::new(),
            interrupts: Interrupts::new(),
            dma: DMAController::new(),
            boot_rom_enable: IORegister::new(),
        }
    }

    pub fn boot_rom_enable(&self) -> u8 {
        self.boot_rom_enable.0
    }

    pub fn interrupts(&self) -> &Interrupts {
        &self.interrupts
    }

    pub fn interrupts_mut(&mut self) -> &mut Interrupts {
        &mut self.interrupts
    }

    pub fn dma(&self) -> &DMAController {
        &self.dma
    }

    pub fn dma_mut(&mut self) -> &mut DMAController {
        &mut self.dma
    }

    pub fn lcd(&self) -> &Lcd {
        &self.lcd
    }

    pub fn lcd_mut(&mut self) -> &mut Lcd {
        &mut self.lcd
    }

    pub fn serial(&self) -> &Serial {
        &self.serial
    }

    pub fn serial_mut(&mut self) -> &mut Serial {
        &mut self.serial
    }

    pub fn timer(&self) -> &Timer {
        &self.timer
    }

    pub fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    pub fn joypad(&self) -> &JoypadInput {
        &self.joypad_input
    }

    pub fn joypad_mut(&mut self) -> &mut JoypadInput {
        &mut self.joypad_input
    }

    pub fn read_u8(&self, address: u16) -> Result<u8, crate::cpu::error::Error> {
        Ok(match address {
            0xFF00 => self.joypad_input.read(),
            0xFF01 => self.serial.read_data(),
            0xFF02 => self.serial.read_control(),
            0xFF04 => self.timer.read_divider(),
            0xFF05 => self.timer.read_timer_counter(),
            0xFF06 => self.timer.read_timer_modulo(),
            0xFF07 => self.timer.read_timer_control(),
            0xFF10 => self.audio.channel_1().read_sweep(),
            0xFF11 => self.audio.channel_1().read_length_timer_and_duty_cycle(),
            0xFF12 => self.audio.channel_1().read_volume_and_envelope(),
            0xFF13 => self.audio.channel_1().read_period_low(),
            0xFF14 => self.audio.channel_1().read_period_high_and_control(),
            0xFF16 => self.audio.channel_2().read_length_timer_and_duty_cycle(),
            0xFF17 => self.audio.channel_2().read_volume_and_envelope(),
            0xFF18 => self.audio.channel_2().read_period_low(),
            0xFF19 => self.audio.channel_2().read_period_high_and_control(),
            0xFF1A => self.audio.channel_3().read_dac_enable(),
            0xFF1B => self.audio.channel_3().read_length_timer(),
            0xFF1C => self.audio.channel_3().read_output_level(),
            0xFF1D => self.audio.channel_3().read_period_low(),
            0xFF1E => self.audio.channel_3().read_period_high_and_control(),
            0xFF20 => self.audio.channel_4().read_length_timer(),
            0xFF21 => self.audio.channel_4().read_volume_and_envelope(),
            0xFF22 => self.audio.channel_4().read_frequency_and_randomness(),
            0xFF23 => self.audio.channel_4().read_control(),
            0xFF30..=0xFF3F => {
                let index = address - 0xFF30;
                self.audio.channel_3().read_wave_pattern_ram(index)
            }
            0xFF24 => self.audio.read_master_volume_vin_panning(),
            0xFF25 => self.audio.read_sound_panning(),
            0xFF26 => self.audio.read_audio_master_control(),
            0xFF40 => self.lcd.read_control(),
            0xFF41 => self.lcd.read_status(),
            0xFF42 => self.lcd.read_scroll_y(),
            0xFF43 => self.lcd.read_scroll_x(),
            0xFF44 => self.lcd.read_lcd_y(),
            0xFF45 => self.lcd.read_lcd_y_compare(),
            0xFF46 => self.dma.read_source_address(),
            0xFF47 => self.lcd.read_background_palette(),
            0xFF48 => self.lcd.read_obj_palette_0(),
            0xFF49 => self.lcd.read_obj_palette_1(),
            0xFF4A => self.lcd.read_window_y(),
            0xFF4B => self.lcd.read_window_x(),
            0xFF50 => self.boot_rom_enable.read(),
            0xFF0F => self.interrupts.read_interrupt_flag(),
            0xFFFF => self.interrupts.read_interrupt_enable(),
            _ => {
                return Err(crate::cpu::error::Error::MemoryReadFault(address));
            }
        })
    }

    pub fn write_u8(&mut self, address: u16, data: u8) -> Result<(), crate::cpu::error::Error> {
        match address {
            0xFF00 => self.joypad_input.write(data),
            0xFF01 => self.serial.write_data(data),
            0xFF02 => self.serial.write_control(data),
            0xFF04 => self.timer.write_divider(data),
            0xFF05 => self.timer.write_timer_counter(data),
            0xFF06 => self.timer.write_timer_modulo(data),
            0xFF07 => self.timer.write_timer_control(data),
            0xFF10 => self.audio.channel_1_mut().write_sweep(data),
            0xFF11 => self
                .audio
                .channel_1_mut()
                .write_length_timer_and_duty_cycle(data),
            0xFF12 => self.audio.channel_1_mut().write_volume_and_envelope(data),
            0xFF13 => self.audio.channel_1_mut().write_period_low(data),
            0xFF14 => self
                .audio
                .channel_1_mut()
                .write_period_high_and_control(data),
            0xFF16 => self
                .audio
                .channel_2_mut()
                .write_length_timer_and_duty_cycle(data),
            0xFF17 => self.audio.channel_2_mut().write_volume_and_envelope(data),
            0xFF18 => self.audio.channel_2_mut().write_period_low(data),
            0xFF19 => self
                .audio
                .channel_2_mut()
                .write_period_high_and_control(data),
            0xFF1A => self.audio.channel_3_mut().write_dac_enable(data),
            0xFF1B => self.audio.channel_3_mut().write_length_timer(data),
            0xFF1C => self.audio.channel_3_mut().write_output_level(data),
            0xFF1D => self.audio.channel_3_mut().write_period_low(data),
            0xFF1E => self
                .audio
                .channel_3_mut()
                .write_period_high_and_control(data),
            0xFF20 => self.audio.channel_4_mut().write_length_timer(data),
            0xFF21 => self.audio.channel_4_mut().write_volume_and_envelope(data),
            0xFF22 => self
                .audio
                .channel_4_mut()
                .write_frequency_and_randomness(data),
            0xFF23 => self.audio.channel_4_mut().write_control(data),
            0xFF30..=0xFF3F => {
                let index = address - 0xFF30;
                self.audio
                    .channel_3_mut()
                    .write_wave_pattern_ram(index, data);
            }
            0xFF24 => self.audio.write_master_volume_vin_panning(data),
            0xFF25 => self.audio.write_sound_panning(data),
            0xFF26 => self.audio.write_audio_master_control(data),
            0xFF40 => self.lcd.write_control(data),
            0xFF41 => self.lcd.write_status(data),
            0xFF42 => self.lcd.write_scroll_y(data),
            0xFF43 => self.lcd.write_scroll_x(data),
            0xFF44 => {} // Writing is not enabled for LCD Y register
            0xFF45 => self.lcd.write_lcd_y_compare(data),
            0xFF46 => self.dma.start_new_transfer(data),
            0xFF47 => self.lcd.write_background_palette(data),
            0xFF48 => self.lcd.write_obj_palette_0(data),
            0xFF49 => self.lcd.write_obj_palette_1(data),
            0xFF4A => self.lcd.write_window_y(data),
            0xFF4B => self.lcd.write_window_x(data),
            0xFF50 => self.boot_rom_enable.write(data),
            0xFF0F => self.interrupts.write_interrupt_flag(data),
            0xFFFF => self.interrupts.write_interrupt_enable(data),
            _ => {
                return Err(crate::cpu::error::Error::MemoryWriteFault(address, data));
            }
        }

        Ok(())
    }
}
