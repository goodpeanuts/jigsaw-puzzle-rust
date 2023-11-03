// ./src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod game;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "puzzle",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(game::Game::new(cc))),
    )
}