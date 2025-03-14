use super::IORegister;

#[derive(Debug, Clone, Copy)]
pub struct Audio {
    audio_master_control: IORegister,
    sound_panning: IORegister,
    master_volume_vin_panning: IORegister,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            audio_master_control: IORegister::new(),
            sound_panning: IORegister::new(),
            master_volume_vin_panning: IORegister::new(),
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
}
