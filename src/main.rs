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

use eframe::egui;
use puzzle::game;

use puzzle::imgs;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_resizable(false),
        hardware_acceleration: eframe::HardwareAcceleration::Required, // 设置是否使用硬件加速
        ..Default::default()                                           // 使用其他默认选项
    };
    eframe::run_native(
        "WHO IS GOODPEANUTS",
        options,
        Box::new(|cc| {
            setup(&cc.egui_ctx);

            // 使用 new 来安全创建 GameApp (现在返回 Result)
            match game::GameApp::new(cc) {
                Ok(app) => Ok(Box::new(app)),
                Err(e) => {
                    eprintln!("Failed to create GameApp: {}", e);
                    // 直接 panic，让 eframe 处理
                    panic!("Failed to initialize game application: {}", e);
                }
            }
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    setup(&cc.egui_ctx);

                    // 使用 new 来安全创建 GameApp (现在返回 Result)
                    match game::GameApp::new(cc) {
                        Ok(app) => Ok(Box::new(app)),
                        Err(e) => {
                            log::error!("Failed to create GameApp: {}", e);
                            // 直接 panic，让 eframe 处理
                            panic!("Failed to initialize game application: {}", e);
                        }
                    }
                }),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

pub fn setup(cc: &egui::Context) {
    egui_extras::install_image_loaders(cc);
    // cc.include_bytes(format!("bytes://background"), imgs::IMAGE_background);
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
