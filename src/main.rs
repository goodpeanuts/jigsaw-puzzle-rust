/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-04 18:35:27
 * @FilePath: \puzzle\src\main.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
// ./src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use puzzle::game;


fn main() -> Result<(), eframe::Error>{
    
    // eframe::run_native(
    //     "puzzle",
    //     eframe::NativeOptions::default(),
    //     Box::new(|cc| Box::new(game::GameApp::new(|c|{
            
    //     }))),
    // )
    eframe::run_native(
        "puzzle",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            // let egui_ctx = &cc.egui_ctx; // Get the egui::Context from CreationContext
            // game::GameApp::setup(&egui_ctx); // Call the setup function here
            //let egui_ctx = &cc.egui_ctx; // Create a new egui::Context
            //egui_extras::install_image_loaders(egui_ctx);
            Box::new(game::GameApp::new(cc)) // Store GameApp in the heap using Box::new
        }),
    )
}
