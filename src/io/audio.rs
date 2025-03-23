use super::IORegister;

#[derive(Debug, Clone, Copy)]
pub struct AudioChannel1 {
    sweep: IORegister,
    length_timer_and_duty_cycle: IORegister,
    volume_and_envelope: IORegister,
    period_low: IORegister,
    period_high_and_control: IORegister,
}

impl AudioChannel1 {
    pub fn new() -> Self {
        Self {
            sweep: IORegister::new(),
            length_timer_and_duty_cycle: IORegister::new(),
            volume_and_envelope: IORegister::new(),
            period_low: IORegister::new(),
            period_high_and_control: IORegister::new(),
        }
    }

    pub fn read_sweep(&self) -> u8 {
        self.sweep.read()
    }

    pub fn write_sweep(&mut self, value: u8) {
        self.sweep.write(value);
    }

    pub fn read_length_timer_and_duty_cycle(&self) -> u8 {
        self.length_timer_and_duty_cycle.read()
    }

    pub fn write_length_timer_and_duty_cycle(&mut self, value: u8) {
        self.length_timer_and_duty_cycle.write(value);
    }

    pub fn read_volume_and_envelope(&self) -> u8 {
        self.volume_and_envelope.read()
    }

    pub fn write_volume_and_envelope(&mut self, value: u8) {
        self.volume_and_envelope.write(value);
    }

    pub fn read_period_low(&self) -> u8 {
        0
    }

    pub fn write_period_low(&mut self, value: u8) {
        self.period_low.write(value);
    }

    pub fn read_period_high_and_control(&self) -> u8 {
        self.period_high_and_control.read()
    }

    pub fn write_period_high_and_control(&mut self, value: u8) {
        self.period_high_and_control.write(value);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AudioChannel2 {
    length_timer_and_duty_cycle: IORegister,
    volume_and_envelope: IORegister,
    period_low: IORegister,
    period_high_and_control: IORegister,
}

impl AudioChannel2 {
    pub fn new() -> Self {
        Self {
            length_timer_and_duty_cycle: IORegister::new(),
            volume_and_envelope: IORegister::new(),
            period_low: IORegister::new(),
            period_high_and_control: IORegister::new(),
        }
    }

    pub fn read_length_timer_and_duty_cycle(&self) -> u8 {
        self.length_timer_and_duty_cycle.read()
    }

    pub fn write_length_timer_and_duty_cycle(&mut self, value: u8) {
        self.length_timer_and_duty_cycle.write(value);
    }

    pub fn read_volume_and_envelope(&self) -> u8 {
        self.volume_and_envelope.read()
    }

    pub fn write_volume_and_envelope(&mut self, value: u8) {
        self.volume_and_envelope.write(value);
    }

    pub fn read_period_low(&self) -> u8 {
        0
    }

    pub fn write_period_low(&mut self, value: u8) {
        self.period_low.write(value);
    }

    pub fn read_period_high_and_control(&self) -> u8 {
        self.period_high_and_control.read()
    }

    pub fn write_period_high_and_control(&mut self, value: u8) {
        self.period_high_and_control.write(value);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AudioChannel3 {
    dac_enable: bool,
    length_timer: IORegister,
    output_level: IORegister,
    period_low: IORegister,
    period_high_and_control: IORegister,
    wave_pattern_ram: [u8; 16]
}

impl AudioChannel3 {
    pub fn new() -> Self {
        Self {
            dac_enable: false,
            length_timer: IORegister::new(),
            output_level: IORegister::new(),
            period_low: IORegister::new(),
            period_high_and_control: IORegister::new(),
            wave_pattern_ram: [0u8; 16]
        }
    }

    pub fn read_dac_enable(&self) -> u8 {
        if self.dac_enable { 1 << 7 } else { 0 }
    }

    pub fn write_dac_enable(&mut self, value: u8) {
        self.dac_enable = (value & (1 << 7)) != 0;
    }

    pub fn read_length_timer(&self) -> u8 {
        self.length_timer.read()
    }

    pub fn write_length_timer(&mut self, value: u8) {
        self.length_timer.write(value);
    }

    pub fn read_output_level(&self) -> u8 {
        self.output_level.read()
    }

    pub fn write_output_level(&mut self, value: u8) {
        self.output_level.write(value);
    }

    pub fn read_period_low(&self) -> u8 {
        0
    }

    pub fn write_period_low(&mut self, value: u8) {
        self.period_low.write(value);
    }

    pub fn read_period_high_and_control(&self) -> u8 {
        self.period_high_and_control.read()
    }

    pub fn write_period_high_and_control(&mut self, value: u8) {
        self.period_high_and_control.write(value);
    }

    pub fn read_wave_pattern_ram(&self, index: u16) -> u8 {
        self.wave_pattern_ram[index as usize]
    }

    pub fn write_wave_pattern_ram(&mut self, index: u16, value: u8) {
        self.wave_pattern_ram[index as usize] = value;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AudioChannel4 {
    length_timer: IORegister,
    volume_and_envelope: IORegister,
    frequency_and_randomness: IORegister,
    control: IORegister,
}

impl AudioChannel4 {
    pub fn new() -> Self {
        Self {
            length_timer: IORegister::new(),
            volume_and_envelope: IORegister::new(),
            frequency_and_randomness: IORegister::new(),
            control: IORegister::new(),
        }
    }


    pub fn read_length_timer(&self) -> u8 {
        0
    }

    pub fn write_length_timer(&mut self, value: u8) {
        self.length_timer.write(value);
    }

    pub fn read_volume_and_envelope(&self) -> u8 {
        self.volume_and_envelope.read()
    }

    pub fn write_volume_and_envelope(&mut self, value: u8) {
        self.volume_and_envelope.write(value);
    }

    pub fn read_frequency_and_randomness(&self) -> u8 {
        self.frequency_and_randomness.read()
    }

    pub fn write_frequency_and_randomness(&mut self, value: u8) {
        self.frequency_and_randomness.write(value);
    }

    pub fn read_control(&self) -> u8 {
        self.control.read()
    }

    pub fn write_control(&mut self, value: u8) {
        self.control.write(value);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Audio {
    audio_master_control: IORegister,
    sound_panning: IORegister,
    master_volume_vin_panning: IORegister,
    channel_1: AudioChannel1,
    channel_2: AudioChannel2,
    channel_3: AudioChannel3,
    channel_4: AudioChannel4,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            audio_master_control: IORegister::new(),
            sound_panning: IORegister::new(),
            master_volume_vin_panning: IORegister::new(),
            channel_1: AudioChannel1::new(),
            channel_2: AudioChannel2::new(),
            channel_3: AudioChannel3::new(),
            channel_4: AudioChannel4::new(),
        }
    }

    pub fn read_audio_master_control(&self) -> u8 {
        self.audio_master_control.read()
    }

    pub fn write_audio_master_control(&mut self, value: u8) {
        self.audio_master_control.write(value);
    }

    pub fn read_sound_panning(&self) -> u8 {
        self.sound_panning.read()
    }

    pub fn write_sound_panning(&mut self, value: u8) {
        self.sound_panning.write(value);
    }

    pub fn read_master_volume_vin_panning(&self) -> u8 {
        self.master_volume_vin_panning.read()
    }

    pub fn write_master_volume_vin_panning(&mut self, value: u8) {
        self.master_volume_vin_panning.write(value);
    }

    pub fn channel_1(&self) -> &AudioChannel1 {
        &self.channel_1
    }

    pub fn channel_1_mut(&mut self) -> &mut AudioChannel1 {
        &mut self.channel_1
    }

    pub fn channel_2(&self) -> &AudioChannel2 {
        &self.channel_2
    }

    pub fn channel_2_mut(&mut self) -> &mut AudioChannel2 {
        &mut self.channel_2
    }

    pub fn channel_3(&self) -> &AudioChannel3 {
        &self.channel_3
    }

    pub fn channel_3_mut(&mut self) -> &mut AudioChannel3 {
        &mut self.channel_3
    }

    pub fn channel_4(&self) -> &AudioChannel4 {
        &self.channel_4
    }

    pub fn channel_4_mut(&mut self) -> &mut AudioChannel4 {
        &mut self.channel_4
    }
}
