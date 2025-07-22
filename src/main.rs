/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-22 16:12:44
 * @FilePath: /jigsaw-puzzle-rust/src/main.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::Mutex;

use puzzle::app;
use puzzle::common::load_images;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_resizable(false),
        hardware_acceleration: eframe::HardwareAcceleration::Required, // 设置是否使用硬件加速
        ..Default::default()                                           // 使用其他默认选项
    };
    eframe::run_native(
        "WHO IS GOODPEANUTS",
        options,
        Box::new(|cc| {
            init_app(&cc.egui_ctx);

            // 使用 new 来安全创建 GameApp (现在返回 Result)
            match app::GameApp::new(cc) {
                Ok(app) => Ok(Box::new(app)),
                Err(e) => {
                    eprintln!("Failed to create GameApp: {e}");
                    // 直接 panic，让 eframe 处理
                    panic!("Failed to initialize game application: {e}");
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
                    init_app(&cc.egui_ctx);

                    // 使用 new 来安全创建 GameApp (现在返回 Result)
                    match app::GameApp::new(cc) {
                        Ok(app) => Ok(Box::new(app)),
                        Err(e) => {
                            log::error!("Failed to create GameApp: {e}");
                            // 直接 panic，让 eframe 处理
                            panic!("Failed to initialize game application: {e}");
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

pub fn init_app(cc: &eframe::egui::Context) {
    load_images(cc);
    let _global_ui_state = puzzle::views::GLOBAL_UI_STATE
        .get_or_init(|| Mutex::new(puzzle::views::GlobalUIState::new(cc)));
}
