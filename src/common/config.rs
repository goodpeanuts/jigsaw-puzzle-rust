/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-18 14:59:28
 * @FilePath: /jigsaw-puzzle-rust/src/common/config.rs
 * @Description: help display chinese characters in egui
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */

#[cfg(feature = "chinese")]
pub fn custom_font(cc: &eframe::CreationContext<'_>) {
    use eframe::egui;
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "OPPOSans-L".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/YaiHe.ttf")).into(),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "OPPOSans-L".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("OPPOSans-L".to_owned());

    // Tell egui to use these fonts:
    cc.egui_ctx.set_fonts(fonts);
}
