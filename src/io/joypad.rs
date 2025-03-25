use crate::InputState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSelection {
    None,
    Buttons,
    DPad,
    Both,
}

#[derive(Clone, Copy)]
pub struct JoypadInput {
    selection: InputSelection,
    inputs: InputState,
    previous_inputs: InputState,
}

impl JoypadInput {
    pub fn new() -> Self {
        Self {
            selection: InputSelection::None,
            inputs: InputState::empty(),
            previous_inputs: InputState::empty(),
        }
    }

    pub fn step(&mut self, input_state: InputState) -> bool {
        self.update_inputs(input_state);
        self.input_changed()
    }

    pub fn input_changed(&self) -> bool {
        let before = self.read_state(self.previous_inputs);
        let now = self.read_state(self.inputs);

        for i in 0..8 {
            let mask = 1 << i;

            if ((before & mask) == 0) & ((now & mask) != 0) {
                return true;
            }
        }

        false
    }

    pub fn update_inputs(&mut self, input_state: InputState) {
        self.previous_inputs = self.inputs;
        self.inputs = input_state;
    }

    pub fn write(&mut self, value: u8) {
        let masked = (!value & 0b0011_0000) >> 4;

        self.selection = match masked {
            0 => InputSelection::None,
            1 => InputSelection::DPad,
            2 => InputSelection::Buttons,
            _ => InputSelection::Both,
        };
    }

    pub fn read(&self) -> u8 {
        self.read_state(self.inputs)
    }

    fn read_state(&self, state: InputState) -> u8 {
        match self.selection {
            InputSelection::Buttons => self.read_buttons(state),
            InputSelection::DPad => self.read_dpad(state),
            InputSelection::Both => self.read_buttons(state) & self.read_dpad(state),
            InputSelection::None => 0b1100_1111,
        }
    }

    fn read_dpad(&self, state: InputState) -> u8 {
        let mut value = 0;
        value |= if state.dpad_state.is_down() {
            0
        } else {
            1 << 3
        };
        value |= if state.dpad_state.is_up() { 0 } else { 1 << 2 };
        value |= if state.dpad_state.is_left() {
            0
        } else {
            1 << 1
        };
        value |= if state.dpad_state.is_right() {
            0
        } else {
            1 << 0
        };
        value
    }

    fn read_buttons(&self, state: InputState) -> u8 {
        let mut value = 0;
        value |= if state.start_pressed { 0 } else { 1 << 3 };
        value |= if state.select_pressed { 0 } else { 1 << 2 };
        value |= if state.b_pressed { 0 } else { 1 << 1 };
        value |= if state.a_pressed { 0 } else { 1 << 0 };
        value
    }
}
