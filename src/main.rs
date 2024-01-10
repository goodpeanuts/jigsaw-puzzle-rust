/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-09 01:28:10
 * @FilePath: \puzzle\src\main.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
// ./src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use puzzle::game;
use eframe::egui;

use puzzle::imgs;

fn main() -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 900.0)), // 设置窗口的宽度和高度
        resizable: false, // 设置窗口是否可以调整大小
        hardware_acceleration: eframe::HardwareAcceleration::Required, // 设置是否使用硬件加速
        ..Default::default() // 使用其他默认选项
    };
    // eframe::run_native(
    //     "puzzle",
    //     eframe::NativeOptions::default(),
    //     Box::new(|cc| Box::new(game::GameApp::new(|c|{
            
    //     }))),
    // )
    eframe::run_native(
        "拼图游戏",
        options,
        Box::new(|cc| {
            setup(&cc.egui_ctx);
            Box::new(game::GameApp::new(cc)) // Store GameApp in the heap using Box::new
        }),
    )
}

pub fn setup(cc: &egui::Context) {
    egui_extras::install_image_loaders(cc);
    //cc.include_bytes(format!("bytes://background"), imgs::IMAGE_background);
    cc.include_bytes(format!("bytes://x{}", 1), imgs::IMAGE_1);
    cc.include_bytes(format!("bytes://x{}", 2), imgs::IMAGE_2);
    cc.include_bytes(format!("bytes://x{}", 3), imgs::IMAGE_3);
    cc.include_bytes(format!("bytes://x{}", 4), imgs::IMAGE_4);
    cc.include_bytes(format!("bytes://x{}", 5), imgs::IMAGE_5);
    cc.include_bytes(format!("bytes://x{}", 6), imgs::IMAGE_6);
    cc.include_bytes(format!("bytes://x{}", 7), imgs::IMAGE_7);
    cc.include_bytes(format!("bytes://x{}", 8), imgs::IMAGE_8);
    cc.include_bytes(format!("bytes://x{}", 9), imgs::IMAGE_9);
    cc.include_bytes(format!("bytes://x{}", 10), imgs::IMAGE_10);
    cc.include_bytes(format!("bytes://x{}", 11), imgs::IMAGE_11);
    cc.include_bytes(format!("bytes://x{}", 12), imgs::IMAGE_12);
}