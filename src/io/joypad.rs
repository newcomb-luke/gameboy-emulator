use crate::InputState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSelection {
    None,
    Buttons,
    DPad
}

#[derive(Clone, Copy)]
pub struct JoypadInput {
    selection: InputSelection,
    inputs: InputState
}

impl JoypadInput {
    pub fn new() -> Self {
        Self {
            selection: InputSelection::None,
            inputs: InputState::empty()
        }
    }

    pub fn set_inputs(&mut self, input_state: InputState) {
        self.inputs = input_state;
    }

    pub fn write(&mut self, value: u8) {
        let masked = (!value & 0b0011_0000) >> 4;

        self.selection = match masked {
            0 => InputSelection::None,
            1 => InputSelection::DPad,
            _ => InputSelection::Buttons,
        };
    }

    pub fn read(&self) -> u8 {
        let mut value = 0;

        match self.selection {
            InputSelection::Buttons => {
                value |= (if self.inputs.dpad_state.is_down() { 0 } else { 1 }) << 3;
                value |= (if self.inputs.dpad_state.is_up() { 0 } else { 1 }) << 2;
                value |= (if self.inputs.dpad_state.is_left() { 0 } else { 1 }) << 1;
                value |= (if self.inputs.dpad_state.is_right() { 0 } else { 1 }) << 0;
            }
            InputSelection::DPad => {
                value |= (if self.inputs.start_pressed { 0 } else { 1 }) << 3;
                value |= (if self.inputs.select_pressed { 0 } else { 1 }) << 2;
                value |= (if self.inputs.b_pressed { 0 } else { 1 }) << 1;
                value |= (if self.inputs.a_pressed { 0 } else { 1 }) << 0;
            }
            InputSelection::None => {}
        }

        value
    }
}