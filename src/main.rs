#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

use std::path::Path;

use eframe::egui::{self, load::SizedTexture, Color32, ColorImage, CornerRadius};
use gameboy_emulator::{
    boot::{BootRom, BootRomReader},
    cartridge::Cartridge,
    cpu::{
        bus::{MainBus, SharedBus},
        Cpu,
    },
    io::SharedIO,
    ppu::{vram::Vram, Ppu},
};

const GAMEBOY_HEIGHT: f32 = 148.0; // mm
const GAMEBOY_WIDTH: f32 = 90.0; // mm
const DISPLAY_HEIGHT: f32 = 47.0; // mm
const DISPLAY_WIDTH: f32 = 43.0; // mm
const SCALE_FACTOR: f32 = 5.0;

fn main() -> eframe::Result {
    let boot_rom = read_boot_rom("dmg_boot.bin");
    let cartridge = read_cartridge("tests/roms/dmg-acid2.gb");

    let header = cartridge.header();

    println!("Title: {}", header.title());
    println!("Manufacturer: {:#?}", header.manufacturer_code());
    println!("Licensee: {}", header.licensee());
    println!("Type: {:#?}", header.cartridge_type());
    println!("ROM Size: {:#?}", header.rom_size());
    println!("RAM Size: {:#?}", header.ram_size());
    println!("Destination code: {:#?}", header.destination_code());
    println!("Version number: {:#?}", header.version_number());
    println!(
        "Header checksum (read): {:02x}",
        header.read_header_checksum()
    );
    println!(
        "Header checksum (computed): {:02x}",
        header.computed_header_checksum()
    );
    println!("Header checksum valid: {}", header.header_checksum_valid());
    println!(
        "Global checksum (read): {:04x}",
        header.read_global_checksum()
    );

    let vram = Vram::new();
    let shared_io = SharedIO::new();

    let bus = MainBus::new(boot_rom, cartridge, vram.clone(), shared_io);
    let shared_bus = SharedBus::new(bus);

    let mut cpu = Cpu::new(shared_bus);

    let mut ppu = Ppu::new(vram);

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
            Ok(Box::new(EmuApp::new(cc)))
        }),
    )
}

struct EmuApp {
    display_texture: egui::TextureHandle,
}

impl EmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let display_image = ColorImage::new([160, 144], Color32::from_rgb(133, 159, 88));

        Self {
            display_texture: cc.egui_ctx.load_texture(
                "display",
                display_image,
                egui::TextureOptions::LINEAR,
            ),
        }
    }
}

impl eframe::App for EmuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

        egui::CentralPanel::default()
            .frame(my_frame)
            .show(ctx, |ui| {
                let display_image = egui::Image::new(SizedTexture::new(
                    &self.display_texture,
                    [DISPLAY_HEIGHT * SCALE_FACTOR, DISPLAY_WIDTH * SCALE_FACTOR],
                ));
                ui.vertical_centered(|ui| {
                    ui.add_space(60.0);
                    ui.add(display_image);
                });
            });
        
        ctx.request_repaint();
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
