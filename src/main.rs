/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-06 01:02:05
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
        ..Default::default() // 使用其他默认选项
    };
    // eframe::run_native(
    //     "puzzle",
    //     eframe::NativeOptions::default(),
    //     Box::new(|cc| Box::new(game::GameApp::new(|c|{
            
    //     }))),
    // )
    eframe::run_native(
        "puzzle",
        options,
        Box::new(|cc| {
            setup(&cc.egui_ctx);
            Box::new(game::GameApp::new(cc)) // Store GameApp in the heap using Box::new
        }),
    )
}

pub fn setup(cc: &egui::Context) {
    egui_extras::install_image_loaders(cc);
    cc.include_bytes(format!("bytes://{}", 100), imgs::IMAGE_1);
    cc.include_bytes(format!("bytes://{}", 99), imgs::IMAGE_2);
    cc.include_bytes(format!("bytes://{}", 98), imgs::IMAGE_3);
    cc.include_bytes(format!("bytes://{}", 97), imgs::IMAGE_4);
    cc.include_bytes(format!("bytes://{}", 96), imgs::IMAGE_5);
    cc.include_bytes(format!("bytes://{}", 95), imgs::IMAGE_6);
    cc.include_bytes(format!("bytes://{}", 94), imgs::IMAGE_7);
    cc.include_bytes(format!("bytes://{}", 93), imgs::IMAGE_8);
    cc.include_bytes(format!("bytes://{}", 92), imgs::IMAGE_9);
    cc.include_bytes(format!("bytes://{}", 91), imgs::IMAGE_10);
    cc.include_bytes(format!("bytes://{}", 90), imgs::IMAGE_11);
    cc.include_bytes(format!("bytes://{}", 89), imgs::IMAGE_12);
}