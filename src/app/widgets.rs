use eframe::{
    egui::{self, Color32, CornerRadius, Rect, Sense, Shape, Ui, Vec2, Widget},
    epaint::RectShape,
};

use crate::{DPadButtonState, DPadState};

use super::DROP_SHADOW;

const AB_BUTTON_COLOR: Color32 = Color32::from_rgb(151, 38, 94);
const AB_BUTTON_CLICKED_COLOR: Color32 = Color32::from_rgb(131, 28, 79);
const START_BUTTON_COLOR: Color32 = Color32::from_rgb(134, 127, 131);
const START_BUTTON_CLICKED_COLOR: Color32 = Color32::from_rgb(124, 117, 121);
const DPAD_BUTTON_COLOR: Color32 = Color32::from_rgb(96, 96, 96);
const DPAD_BUTTON_CLICKED_COLOR: Color32 = Color32::from_rgb(86, 86, 86);

pub struct ABButton {
    activated: bool,
}

impl ABButton {
    pub fn new(activated: bool) -> Self {
        Self { activated }
    }
}

impl Widget for ABButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let outer_rect_bounds = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(outer_rect_bounds, Sense::drag());
        let interacted = response.dragged();

        let fill_color = if interacted | self.activated {
            AB_BUTTON_CLICKED_COLOR
        } else {
            AB_BUTTON_COLOR
        };

        let button = Shape::circle_filled(
            outer_rect_bounds.center(),
            outer_rect_bounds.width() / 2.0,
            fill_color,
        );

        let shape = if interacted | self.activated {
            button
        } else {
            let shadow = DROP_SHADOW.as_shape(outer_rect_bounds, CornerRadius::same(255));
            Shape::Vec(vec![Shape::from(shadow), button])
        };

        if ui.is_rect_visible(outer_rect_bounds) {
            ui.painter().add(shape);
        }

        response
    }
}

pub struct StartButton {
    activated: bool,
}

impl StartButton {
    pub fn new(activated: bool) -> Self {
        Self { activated }
    }
}

impl Widget for StartButton {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let outer_rect_bounds = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(outer_rect_bounds, Sense::drag());
        let interacted = response.dragged();

        let fill_color = if interacted | self.activated {
            START_BUTTON_CLICKED_COLOR
        } else {
            START_BUTTON_COLOR
        };

        let button = Shape::from(RectShape::filled(
            outer_rect_bounds,
            CornerRadius::same(4),
            fill_color,
        ));

        let shape = if interacted | self.activated {
            button
        } else {
            let shadow = DROP_SHADOW.as_shape(outer_rect_bounds, CornerRadius::same(4));
            Shape::Vec(vec![Shape::from(shadow), button])
        };

        if ui.is_rect_visible(outer_rect_bounds) {
            ui.painter().add(shape);
        }

        response
    }
}

pub struct DPad {
    pub keyboard_input_state: DPadButtonState,
    pub state: DPadState,
}

impl DPad {
    pub fn new() -> Self {
        Self {
            keyboard_input_state: DPadButtonState::empty(),
            state: DPadState::None,
        }
    }
}

impl Widget for &mut DPad {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let outer_rect_bounds = ui.available_rect_before_wrap();
        let overall_response = ui.allocate_rect(outer_rect_bounds, Sense::drag());

        let (left_rect, rest) = outer_rect_bounds
            .scale_from_center2(Vec2::new(1.0, 0.3))
            .split_left_right_at_fraction(0.4);
        let (center_rect, right_rect) = rest.split_left_right_at_fraction(1.0 / 3.0);
        let (top_rect, rest) = outer_rect_bounds
            .scale_from_center2(Vec2::new(0.3, 1.0))
            .split_top_bottom_at_fraction(0.4);
        let (_, bottom_rect) = rest.split_top_bottom_at_fraction(1.0 / 3.0);

        let left_response = ui.allocate_rect(left_rect, Sense::drag());
        let right_response = ui.allocate_rect(right_rect, Sense::drag());
        let top_response = ui.allocate_rect(top_rect, Sense::drag());
        let bottom_response = ui.allocate_rect(bottom_rect, Sense::drag());

        let (top, rest) = outer_rect_bounds.split_top_bottom_at_fraction(0.35);
        let (_, bottom) = rest.split_top_bottom_at_fraction(0.5);
        let (top_left, rest) = top.split_left_right_at_fraction(0.35);
        let (_, top_right) = rest.split_left_right_at_fraction(0.5);
        let (bottom_left, rest) = bottom.split_left_right_at_fraction(0.35);
        let (_, bottom_right) = rest.split_left_right_at_fraction(0.5);

        let top_left_corner = ui.allocate_rect(top_left, Sense::drag());
        let top_right_corner = ui.allocate_rect(top_right, Sense::drag());
        let bottom_left_corner = ui.allocate_rect(bottom_left, Sense::drag());
        let bottom_right_corner = ui.allocate_rect(bottom_right, Sense::drag());

        let left_activated =
            top_left_corner.dragged() | bottom_left_corner.dragged() | left_response.dragged();
        let right_activated =
            top_right_corner.dragged() | bottom_right_corner.dragged() | right_response.dragged();
        let top_activated =
            top_left_corner.dragged() | top_right_corner.dragged() | top_response.dragged();
        let bottom_activated = bottom_left_corner.dragged()
            | bottom_right_corner.dragged()
            | bottom_response.dragged();

        let ui_state = DPadButtonState::new(
            top_activated,
            bottom_activated,
            left_activated,
            right_activated,
        );
        let overall_state = self.keyboard_input_state | ui_state;

        let dpad_state = DPadState::from_buttons(overall_state);
        self.state = dpad_state;

        let mut shadows = Vec::new();
        let mut buttons = Vec::new();

        let center = Shape::from(RectShape::filled(
            center_rect,
            CornerRadius::ZERO,
            DPAD_BUTTON_COLOR,
        ));
        buttons.push(center);

        self.paint_button(left_rect, &mut buttons, &mut shadows, dpad_state.is_left());
        self.paint_button(
            right_rect,
            &mut buttons,
            &mut shadows,
            dpad_state.is_right(),
        );
        self.paint_button(top_rect, &mut buttons, &mut shadows, dpad_state.is_up());
        self.paint_button(
            bottom_rect,
            &mut buttons,
            &mut shadows,
            dpad_state.is_down(),
        );

        if ui.is_rect_visible(outer_rect_bounds) {
            let shadows = Shape::Vec(shadows);
            let buttons = Shape::Vec(buttons);

            ui.painter().add(Shape::Vec(vec![shadows, buttons]));
        }

        overall_response
    }
}

impl DPad {
    fn paint_button(
        &mut self,
        rect: Rect,
        buttons: &mut Vec<Shape>,
        shadows: &mut Vec<Shape>,
        activation_override: bool,
    ) {
        let corner_radius = CornerRadius::same(2);

        let fill_color = if activation_override {
            DPAD_BUTTON_CLICKED_COLOR
        } else {
            DPAD_BUTTON_COLOR
        };

        if !activation_override {
            let shadow = DROP_SHADOW.as_shape(rect, corner_radius);
            let shape = Shape::from(shadow);
            shadows.push(shape);
        }

        let button = Shape::from(RectShape::filled(rect, corner_radius, fill_color));

        buttons.push(button);
    }
}
