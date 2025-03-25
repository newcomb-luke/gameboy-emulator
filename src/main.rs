#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

use std::path::PathBuf;

use clap::Parser;
use eframe::{
    egui::{
        self, load::SizedTexture, text::LayoutJob, Color32, ColorImage, CornerRadius, FontId,
        Label, Margin, Pos2, Rect, Sense, Shadow, Shape, TextFormat, Ui, Vec2, Widget,
    },
    epaint::{
        text::{FontInsert, InsertFontFamily},
        RectShape,
    },
};
use gameboy_emulator::{
    boot::DEFAULT_BOOT_ROM,
    ppu::{DISPLAY_SIZE_PIXELS, OFF_COLOR},
    read_boot_rom, read_cartridge, DPadButtonState, DPadState, Emulator, InputState,
};

const GAMEBOY_HEIGHT: f32 = 148.0; // mm
const GAMEBOY_WIDTH: f32 = 90.0; // mm
const DISPLAY_HEIGHT: f32 = 47.0; // mm
const DISPLAY_WIDTH: f32 = 43.0; // mm
const SCALE_FACTOR: f32 = 6.0;
const SCALED_GAMEBOY_HEIGHT: f32 = GAMEBOY_HEIGHT * SCALE_FACTOR;
const SCALED_GAMEBOY_WIDTH: f32 = GAMEBOY_WIDTH * SCALE_FACTOR;
const BUTTON_DIAMETER: f32 = 10.0; // mm
const SCALED_BUTTON_DIAMETER: f32 = BUTTON_DIAMETER * SCALE_FACTOR;
const DPAD_WIDTH: f32 = 20.0; // mm
const SCALED_DPAD_WIDTH: f32 = DPAD_WIDTH * SCALE_FACTOR;

const GAMEBOY_COLOR: Color32 = Color32::from_rgb(193, 189, 186);
const DISPLAY_FRAME_COLOR: Color32 = Color32::from_rgb(98, 95, 114);
const FONT_COLOR: Color32 = Color32::from_rgb(67, 67, 142);
const AB_BUTTON_COLOR: Color32 = Color32::from_rgb(151, 38, 94);
const AB_BUTTON_CLICKED_COLOR: Color32 = Color32::from_rgb(131, 28, 79);
const START_BUTTON_COLOR: Color32 = Color32::from_rgb(134, 127, 131);
const START_BUTTON_CLICKED_COLOR: Color32 = Color32::from_rgb(124, 117, 121);
const DPAD_BUTTON_COLOR: Color32 = Color32::from_rgb(96, 96, 96);
const DPAD_BUTTON_CLICKED_COLOR: Color32 = Color32::from_rgb(86, 86, 86);
const DROP_SHADOW: Shadow = Shadow {
    offset: [4, 4],
    blur: 2,
    spread: 0,
    color: Color32::GRAY,
};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(
        value_name = "ROM_PATH",
        help = "The path to the ROM which will be loaded as a cartridge"
    )]
    cartridge_rom_path: PathBuf,
    #[arg(
        short = 'b',
        long = "boot-rom",
        help = "The path to a boot ROM to use to start the GameBoy. Optional."
    )]
    boot_rom_path: Option<PathBuf>,
}

fn main() -> eframe::Result {
    let args = Args::parse();

    let boot_rom = if let Some(path) = args.boot_rom_path {
        read_boot_rom(&path)
    } else {
        DEFAULT_BOOT_ROM
    };

    let cartridge = read_cartridge(&args.cartridge_rom_path);

    let emulator = Emulator::new(boot_rom, cartridge);

    // emulator.add_breakpoint(0x0000);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([SCALED_GAMEBOY_WIDTH, SCALED_GAMEBOY_HEIGHT]),
        ..Default::default()
    };

    eframe::run_native(
        "gameboy-emulator",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(EmuApp::new(cc, emulator)))
        }),
    )
}

struct EmuApp {
    emulator: Emulator,
    display_texture: egui::TextureHandle,
    breakpoint_reached: bool,
    input_state: InputState,
    dpad: DPad,
}

impl eframe::App for EmuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|input| {
            let arrow_up = input.key_down(egui::Key::ArrowUp);
            let arrow_down = input.key_down(egui::Key::ArrowDown);
            let arrow_left = input.key_down(egui::Key::ArrowLeft);
            let arrow_right = input.key_down(egui::Key::ArrowRight);

            let a_button = input.key_down(egui::Key::X);
            let b_button = input.key_down(egui::Key::Z);

            let start_button = input.key_down(egui::Key::Enter);
            let select_button = input.key_down(egui::Key::Backspace);

            self.input_state.a_pressed = a_button;
            self.input_state.b_pressed = b_button;
            self.input_state.select_pressed = select_button;
            self.input_state.start_pressed = start_button;
            self.dpad.keyboard_input_state =
                DPadButtonState::new(arrow_up, arrow_down, arrow_left, arrow_right);
        });

        self.show_gameboy(ctx, self.breakpoint_reached);

        self.input_state.dpad_state = self.dpad.state;

        self.breakpoint_reached = false;

        let mut cycles_done = 0;

        const CYCLES_PER_FRAME: usize = 69905;

        while cycles_done < CYCLES_PER_FRAME {
            if let Some(_) = self.emulator.breakpoint_reached() {
                self.breakpoint_reached = true;
                break;
            } else {
                let (cycles, new_frame) = self.emulator.step(self.input_state).unwrap();
                cycles_done += cycles;

                if new_frame {
                    let pixels = self.emulator.get_pixels();

                    self.display_texture.set(
                        egui::ColorImage {
                            size: *DISPLAY_SIZE_PIXELS,
                            pixels: pixels.to_vec(),
                        },
                        egui::TextureOptions::NEAREST,
                    );
                }
            }
        }

        ctx.request_repaint();
    }
}

impl EmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>, emulator: Emulator) -> Self {
        let display_image = ColorImage::new(*DISPLAY_SIZE_PIXELS, OFF_COLOR);

        cc.egui_ctx.add_font(FontInsert::new(
            "Corporate",
            egui::FontData::from_static(include_bytes!("fonts/av05-logotype.ttf")),
            vec![InsertFontFamily {
                family: egui::FontFamily::Name("Corporate".into()),
                priority: egui::epaint::text::FontPriority::Lowest,
            }],
        ));

        cc.egui_ctx.style_mut(|style| {
            style.interaction.selectable_labels = false;
        });

        Self {
            emulator,
            display_texture: cc.egui_ctx.load_texture(
                "display",
                display_image,
                egui::TextureOptions::NEAREST,
            ),
            breakpoint_reached: false,
            input_state: InputState::empty(),
            dpad: DPad::new(),
        }
    }

    fn show_gameboy(&mut self, ctx: &egui::Context, breakpoint_reached: bool) {
        let gameboy_outline = egui::containers::Frame {
            outer_margin: egui::Margin::same(10),
            inner_margin: egui::Margin::ZERO,
            corner_radius: CornerRadius::same(10),
            shadow: eframe::epaint::Shadow::NONE,
            fill: GAMEBOY_COLOR,
            stroke: egui::Stroke::new(2.0, Color32::GRAY),
        };

        egui::CentralPanel::default()
            .frame(gameboy_outline)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if breakpoint_reached {
                        ui.label("Breakpoint reached.");
                    }

                    ui.add_space(20.0);

                    self.show_display(ui);
                    self.show_buttons(ui);
                });
            });
    }

    fn show_display(&mut self, ui: &mut Ui) {
        let display_image = egui::Image::new(SizedTexture::new(
            &self.display_texture,
            [
                DISPLAY_HEIGHT * (SCALE_FACTOR + 2.0),
                DISPLAY_WIDTH * (SCALE_FACTOR + 2.0),
            ],
        ));

        egui::Frame::default()
            .outer_margin(Margin::same(20))
            .inner_margin(Margin::same(20))
            .shadow(DROP_SHADOW)
            .fill(DISPLAY_FRAME_COLOR)
            .corner_radius(CornerRadius {
                nw: 15,
                ne: 15,
                sw: 15,
                se: 50,
            })
            .show(ui, |ui| {
                ui.add(display_image);
            });
    }

    fn show_buttons(&mut self, ui: &mut Ui) {
        const A_POS: Pos2 = Pos2::new(SCALED_GAMEBOY_WIDTH * 0.88, SCALED_GAMEBOY_HEIGHT * 0.65);
        const B_POS: Pos2 = Pos2::new(SCALED_GAMEBOY_WIDTH * 0.72, SCALED_GAMEBOY_HEIGHT * 0.70);

        let a_clicked = self.show_ab_button(ui, "A", A_POS, self.input_state.a_pressed);
        let b_clicked = self.show_ab_button(ui, "B", B_POS, self.input_state.b_pressed);

        if !self.input_state.a_pressed {
            self.input_state.a_pressed = a_clicked;
        }

        if !self.input_state.b_pressed {
            self.input_state.b_pressed = b_clicked;
        }

        const START_POS: Pos2 =
            Pos2::new(SCALED_GAMEBOY_WIDTH * 0.62, SCALED_GAMEBOY_HEIGHT * 0.83);
        const SELECT_POS: Pos2 =
            Pos2::new(SCALED_GAMEBOY_WIDTH * 0.42, SCALED_GAMEBOY_HEIGHT * 0.83);

        let start_clicked =
            self.show_start_button(ui, "START", START_POS, self.input_state.start_pressed);
        let select_clicked =
            self.show_start_button(ui, "SELECT", SELECT_POS, self.input_state.select_pressed);

        if !self.input_state.start_pressed {
            self.input_state.start_pressed = start_clicked;
        }

        if !self.input_state.select_pressed {
            self.input_state.select_pressed = select_clicked;
        }

        const DPAD_POS: Pos2 = Pos2::new(SCALED_GAMEBOY_WIDTH * 0.19, SCALED_GAMEBOY_HEIGHT * 0.67);

        self.show_dpad(ui, DPAD_POS);
    }

    fn show_ab_button(
        &mut self,
        ui: &mut Ui,
        text: &str,
        pos: Pos2,
        activation_override: bool,
    ) -> bool {
        let font_id = FontId::new(24.0, egui::FontFamily::Name("Corporate".into()));
        const BUTTON_SIZE: Vec2 = Vec2::new(SCALED_BUTTON_DIAMETER, SCALED_BUTTON_DIAMETER);

        let clicked = ui
            .put(
                Rect::from_center_size(pos, BUTTON_SIZE),
                ABButton::new(activation_override),
            )
            .dragged();

        let text_pos = Pos2::new(pos.x, pos.y + 50.0);

        let b_text_center = Rect::from_center_size(text_pos, BUTTON_SIZE);

        let mut b_button = LayoutJob::default();
        b_button.append(text, 0.0, TextFormat::simple(font_id, FONT_COLOR));

        ui.put(b_text_center, Label::new(b_button));

        clicked
    }

    fn show_start_button(
        &mut self,
        ui: &mut Ui,
        text: &str,
        pos: Pos2,
        activation_override: bool,
    ) -> bool {
        const BUTTON_SIZE: Vec2 = Vec2::new(SCALED_BUTTON_DIAMETER, SCALED_BUTTON_DIAMETER * 0.3);

        let clicked = ui
            .put(
                Rect::from_center_size(pos, BUTTON_SIZE),
                StartButton::new(activation_override),
            )
            .dragged();

        let text_pos = Pos2::new(pos.x, pos.y + 30.0);
        let text_center = Rect::from_center_size(text_pos, BUTTON_SIZE * 2.0);

        let mut button = LayoutJob::default();
        let font_id = FontId::new(24.0, egui::FontFamily::Name("Corporate".into()));
        button.append(text, 0.0, TextFormat::simple(font_id, FONT_COLOR));

        ui.put(text_center, Label::new(button));

        clicked
    }

    fn show_dpad(&mut self, ui: &mut Ui, pos: Pos2) {
        const DPAD_SIZE: Vec2 = Vec2::new(SCALED_DPAD_WIDTH, SCALED_DPAD_WIDTH);

        ui.put(Rect::from_center_size(pos, DPAD_SIZE), &mut self.dpad);
    }
}

struct ABButton {
    activated: bool,
}

impl ABButton {
    fn new(activated: bool) -> Self {
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

struct StartButton {
    activated: bool,
}

impl StartButton {
    fn new(activated: bool) -> Self {
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

struct DPad {
    keyboard_input_state: DPadButtonState,
    state: DPadState,
}

impl DPad {
    fn new() -> Self {
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
