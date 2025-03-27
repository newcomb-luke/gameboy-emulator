#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

use std::path::PathBuf;

use clap::Parser;
use eframe::egui;
use gameboy_emulator::{
    app::{EmuApp, SCALED_GAMEBOY_HEIGHT, SCALED_GAMEBOY_WIDTH},
    boot::DEFAULT_BOOT_ROM,
    read_boot_rom, read_cartridge, Emulator,
};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(
        short = 'r',
        long = "rom-path",
        help = "The path to the ROM which will be loaded as a cartridge"
    )]
    cartridge_rom_path: Option<PathBuf>,
    #[arg(
        short = 'b',
        long = "boot-rom",
        help = "The path to a boot ROM to use to start the GameBoy"
    )]
    boot_rom_path: Option<PathBuf>,
}

fn main() -> eframe::Result {
    let args = Args::parse();

    let boot_rom = if let Some(boot_rom_path) = args.boot_rom_path {
        match read_boot_rom(boot_rom_path) {
            Ok(boot_rom) => boot_rom,
            Err(e) => {
                eprintln!("Error while loading boot ROM: {e:?}");
                std::process::exit(1);
            }
        }
    } else {
        DEFAULT_BOOT_ROM
    };

    let emulator = if let Some(cartridge_path) = args.cartridge_rom_path {
        match read_cartridge(cartridge_path) {
            Ok(cartridge) => Some(Emulator::new(boot_rom, cartridge)),
            Err(e) => {
                eprintln!("Error while loading cartridge: {e:?}");
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_inner_size([SCALED_GAMEBOY_WIDTH, SCALED_GAMEBOY_HEIGHT]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Dotra Emulator",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(EmuApp::new(cc, boot_rom, emulator)))
        }),
    )
}
