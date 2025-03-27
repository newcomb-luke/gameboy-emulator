use std::path::PathBuf;

use eframe::{
    egui::{
        self, load::SizedTexture, text::LayoutJob, Color32, ColorImage, CornerRadius, FontId,
        Label, Margin, Pos2, Rect, Shadow, TextFormat, Ui, Vec2,
    },
    epaint::text::{FontInsert, InsertFontFamily},
};
use native_dialog::{FileDialog, MessageDialog};
use widgets::{ABButton, DPad, StartButton};

use crate::{
    boot::BootRom,
    config::{get_recents, save_recents, Recents, RomEntry},
    ppu::{DISPLAY_SIZE_PIXELS, OFF_COLOR},
    read_boot_rom, read_cartridge, DPadButtonState, Emulator, InputState,
};

mod widgets;

const GAMEBOY_HEIGHT: f32 = 148.0; // mm
const GAMEBOY_WIDTH: f32 = 90.0; // mm
const DISPLAY_HEIGHT: f32 = 47.0; // mm
const DISPLAY_WIDTH: f32 = 43.0; // mm
const SCALE_FACTOR: f32 = 6.0;
pub const SCALED_GAMEBOY_HEIGHT: f32 = GAMEBOY_HEIGHT * SCALE_FACTOR;
pub const SCALED_GAMEBOY_WIDTH: f32 = GAMEBOY_WIDTH * SCALE_FACTOR;
const BUTTON_DIAMETER: f32 = 10.0; // mm
const SCALED_BUTTON_DIAMETER: f32 = BUTTON_DIAMETER * SCALE_FACTOR;
const DPAD_WIDTH: f32 = 20.0; // mm
const SCALED_DPAD_WIDTH: f32 = DPAD_WIDTH * SCALE_FACTOR;

const GAMEBOY_BODY_COLOR: Color32 = Color32::from_rgb(193, 189, 186);
const DISPLAY_FRAME_COLOR: Color32 = Color32::from_rgb(98, 95, 114);
const FONT_COLOR: Color32 = Color32::from_rgb(67, 67, 142);
pub(crate) const DROP_SHADOW: Shadow = Shadow {
    offset: [4, 4],
    blur: 2,
    spread: 0,
    color: Color32::GRAY,
};

const CYCLES_PER_FRAME: usize = 69905;

pub struct EmuApp {
    emulator: Option<Emulator>,
    display_texture: egui::TextureHandle,
    breakpoint_reached: bool,
    input_state: InputState,
    dpad: DPad,
    boot_rom: BootRom,
    recents: Recents,
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.menu_bar(ui);
        });

        self.show_gameboy(ctx, self.breakpoint_reached);

        self.run_emulator();

        let window_scale_factor = ctx.native_pixels_per_point().unwrap_or(1.0);
        ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(Vec2::from([
            SCALED_GAMEBOY_WIDTH / window_scale_factor,
            SCALED_GAMEBOY_HEIGHT / window_scale_factor,
        ])));

        ctx.request_repaint();
    }
}

impl EmuApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        boot_rom: BootRom,
        emulator: Option<Emulator>,
    ) -> Self {
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

        let recents = get_recents();

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
            boot_rom,
            recents,
        }
    }

    fn run_emulator(&mut self) {
        if let Some(emulator) = &mut self.emulator {
            self.input_state.dpad_state = self.dpad.state;
            self.breakpoint_reached = false;

            let mut cycles_done = 0;

            while cycles_done < CYCLES_PER_FRAME {
                if let Some(_) = emulator.breakpoint_reached() {
                    self.breakpoint_reached = true;
                    break;
                } else {
                    let (cycles, new_frame) = emulator.step(self.input_state).unwrap();
                    cycles_done += cycles;

                    if new_frame {
                        let pixels = emulator.get_pixels();

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
        }
    }

    fn menu_bar(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let path = choose_cartridge_file_with_dialog();

                    if let Some(path) = path {
                        match read_cartridge(&path) {
                            Ok(cartridge) => {
                                let name = cartridge.header().title();
                                let entry = RomEntry::new(name, path);

                                self.recents.add_if_not_present(entry);
                                save_recents(&self.recents);

                                self.emulator = None;
                                self.emulator = Some(Emulator::new(self.boot_rom, cartridge));
                            },
                            Err(e) => {
                                let error_text = match e {
                                    crate::cartridge::Error::UnsupportedCartridgeType => {
                                        "Unsupported cartridge type"
                                    },
                                    _ => {
                                        "Invalid cartridge format. Be sure to choose the correct file"
                                    }
                                };

                                MessageDialog::new()
                                    .set_title("Error reading cartridge")
                                    .set_text(error_text)
                                    .set_type(native_dialog::MessageType::Error)
                                    .show_alert().unwrap();
                            }
                        }
                    }

                    ui.close_menu();
                }

                let recents_menu_enabled = !self.recents.roms().is_empty();
                ui.add_enabled_ui(recents_menu_enabled, |ui| {
                    ui.menu_button("Open Recent", |ui| {
                        for recent in self.recents.roms() {
                            let file_name = recent.path().file_name().unwrap().to_string_lossy();
                            if ui.button(format!("{} - {}", recent.name(), file_name)).clicked() {
                                match read_cartridge(recent.path()) {
                                    Ok(cartridge) => {
                                        self.emulator = None;
                                        self.emulator = Some(Emulator::new(self.boot_rom, cartridge));
                                    },
                                    Err(_) => {
                                        let error_text = format!("Unable to read cartridge `{}` at {}", recent.name(), recent.path().display());
                                        MessageDialog::new()
                                            .set_title("Error reading cartridge")
                                            .set_text(&error_text)
                                            .set_type(native_dialog::MessageType::Error)
                                            .show_alert().unwrap();
                                    }
                                }

                                ui.close_menu();
                            }
                        }
                    });
                });
                if ui.button("Choose Boot ROM").clicked() {
                    let path = choose_boot_rom_file_with_dialog();

                    if let Some(path) = path {
                        match read_boot_rom(path) {
                            Ok(boot_rom) => {
                                self.boot_rom = boot_rom;
                            },
                            Err(_) => {
                                MessageDialog::new()
                                    .set_title("Error reading boot ROM")
                                    .set_text("Invalid file size or format")
                                    .set_type(native_dialog::MessageType::Error)
                                    .show_alert().unwrap();
                            }
                        }
                    }

                    ui.close_menu();
                }
            });
        });
    }

    fn show_gameboy(&mut self, ctx: &egui::Context, breakpoint_reached: bool) {
        let gameboy_outline = egui::containers::Frame {
            outer_margin: egui::Margin::same(10),
            inner_margin: egui::Margin::ZERO,
            corner_radius: CornerRadius::same(10),
            shadow: eframe::epaint::Shadow::NONE,
            fill: GAMEBOY_BODY_COLOR,
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

                    // ui.add(egui::Image::new(egui::include_image!("images/logo.png")));

                    self.show_buttons(ui);
                });
            });
    }

    fn show_display(&mut self, ui: &mut Ui) {
        let display_image = egui::Image::new(SizedTexture::new(
            &self.display_texture,
            [DISPLAY_HEIGHT * SCALE_FACTOR, DISPLAY_WIDTH * SCALE_FACTOR],
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

fn choose_cartridge_file_with_dialog() -> Option<PathBuf> {
    let dialog_result = FileDialog::new()
        .add_filter("GameBoy cartridge file", &["gb"])
        .show_open_single_file();

    match dialog_result {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error occured while displaying file chooser: {e}");
            std::process::exit(1);
        }
    }
}

fn choose_boot_rom_file_with_dialog() -> Option<PathBuf> {
    let dialog_result = FileDialog::new()
        .add_filter("GameBoy boot ROM file", &["bin"])
        .show_open_single_file();

    match dialog_result {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error occured while displaying file chooser: {e}");
            std::process::exit(1);
        }
    }
}
