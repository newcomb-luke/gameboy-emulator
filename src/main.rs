#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

use std::{path::{Path, PathBuf}, time::Instant};

use clap::Parser;
use eframe::egui::{self, load::SizedTexture, Color32, ColorImage, CornerRadius};
use gameboy_emulator::{
    boot::{BootRom, BootRomReader},
    cartridge::Cartridge,
    Emulator, DISPLAY_SIZE_PIXELS,
};

const GAMEBOY_HEIGHT: f32 = 148.0; // mm
const GAMEBOY_WIDTH: f32 = 90.0; // mm
const DISPLAY_HEIGHT: f32 = 47.0; // mm
const DISPLAY_WIDTH: f32 = 43.0; // mm
const SCALE_FACTOR: f32 = 6.0;

const DEFAULT_BOOT_ROM: BootRom = BootRom::new(*include_bytes!("binaries/dmg_boot.bin"));

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

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([GAMEBOY_WIDTH * SCALE_FACTOR, GAMEBOY_HEIGHT * SCALE_FACTOR]),
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
    last_frame_time: Instant,
}

impl EmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>, emulator: Emulator) -> Self {
        let display_image = ColorImage::new(*DISPLAY_SIZE_PIXELS, Color32::from_rgb(133, 159, 88));

        Self {
            emulator,
            display_texture: cc.egui_ctx.load_texture(
                "display",
                display_image,
                egui::TextureOptions::NEAREST,
            ),
            last_frame_time: Instant::now()
        }
    }
}

impl eframe::App for EmuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let delta = now - self.last_frame_time;
        let fps = 1.0 / delta.as_secs_f32();

        let my_frame = egui::containers::Frame {
            outer_margin: egui::Margin {
                left: 10,
                right: 10,
                top: 10,
                bottom: 10,
            },
            inner_margin: egui::Margin::ZERO,
            corner_radius: CornerRadius::same(10),
            shadow: eframe::epaint::Shadow::NONE,
            fill: Color32::from_rgb(193, 189, 186),
            stroke: egui::Stroke::new(2.0, Color32::GRAY),
        };

        let mut breakpoint_reached = false;

        for i in 0..200 {
            if let Some(_) = self.emulator.breakpoint_reached() {
                breakpoint_reached = true;
                break;
            } else {
                self.emulator.step().unwrap();

                if (i % 4) == 0 {
                    let pixels = self.emulator.get_pixels();

                    self.display_texture.set(
                        egui::ColorImage {
                            size: *DISPLAY_SIZE_PIXELS,
                            pixels,
                        },
                        egui::TextureOptions::NEAREST,
                    );
                }
            }
        }

        egui::CentralPanel::default()
            .frame(my_frame)
            .show(ctx, |ui| {
                let display_image = egui::Image::new(SizedTexture::new(
                    &self.display_texture,
                    [
                        DISPLAY_HEIGHT * (SCALE_FACTOR + 2.0),
                        DISPLAY_WIDTH * (SCALE_FACTOR + 2.0),
                    ],
                ));
                ui.vertical_centered(|ui| {
                    if breakpoint_reached {
                        ui.label("Breakpoint reached.");
                    }

                    ui.label(format!("FPS: {:.2}", fps));

                    let state = self.emulator.execution_state();
                    ui.label(format!("{}", state));

                    ui.add_space(60.0);
                    ui.add(display_image);
                });
            });

        ctx.request_repaint();

        self.last_frame_time = now;
    }
}

fn read_cartridge<P>(path: P) -> Cartridge
where
    P: AsRef<Path>,
{
    let mut cartridge_file = std::fs::File::open(path).unwrap();
    Cartridge::read(&mut cartridge_file).unwrap()
}

fn read_boot_rom<P>(path: P) -> BootRom
where
    P: AsRef<Path>,
{
    let mut boot_rom_file = std::fs::File::open(path).unwrap();
    BootRomReader::read(&mut boot_rom_file).unwrap()
}
