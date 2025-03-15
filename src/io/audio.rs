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
pub struct Audio {
    audio_master_control: IORegister,
    sound_panning: IORegister,
    master_volume_vin_panning: IORegister,
    channel_1: AudioChannel1,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            audio_master_control: IORegister::new(),
            sound_panning: IORegister::new(),
            master_volume_vin_panning: IORegister::new(),
            channel_1: AudioChannel1::new(),
        }
    }

    pub fn read_audio_master_control(&mut self) -> u8 {
        self.audio_master_control.read()
    }

    pub fn write_audio_master_control(&mut self, value: u8) {
        self.audio_master_control.write(value);
    }

    pub fn read_sound_panning(&mut self) -> u8 {
        self.sound_panning.read()
    }

    pub fn write_sound_panning(&mut self, value: u8) {
        self.sound_panning.write(value);
    }

    pub fn read_master_volume_vin_panning(&mut self) -> u8 {
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
}
