/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-07 10:31:27
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-22 10:20:24
 * @FilePath: /jigsaw-puzzle-rust/src/views/sidebar.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */

use crate::{app::GameApp, common::time::TimeDelta};
use eframe::egui::{self, Button};
use std::sync::Mutex;

// Constants for font size calculation
const FONT_SIZE_BASE: f32 = 330.0;
const FONT_SIZE_DIVISOR: f32 = 0.6;

// 这里的 SHOW_ORIGIN_IMAGE 如果设置在 GameApp 中，调用self.show_origin_image函数时会造成对变量的多次可变引用
// 如果定义直接定义在 self.game_side 会导致每次调用 self.game_side 都会重新初始化 SHOW_ORIGIN_IMAGE
// 如果想要设置一个可变全局静态变量，可以使用lazy_static宏，并且需要加锁访问
// 此外，设置其值时需要解锁，因此创建一个函数来设置其值
lazy_static::lazy_static! {
    static ref SHOW_ORIGIN_IMAGE: Mutex<bool> = Mutex::new(false);
}

fn set_show_origin_image(value: bool) {
    let mut show_origin_image = SHOW_ORIGIN_IMAGE.lock().unwrap();
    *show_origin_image = value;
}

impl GameApp {
    fn show_origin_image(&mut self, ctx: &egui::Context, _ui: &mut egui::Ui, is_open: &mut bool) {
        egui::Window::new("Original image")
            .title_bar(true)
            .open(is_open)
            .default_open(true)
            .constrain(true)
            .collapsible(false)
            .movable(true)
            .show(ctx, |ui| {
                ui.add(egui::Image::from_uri(self.img().get_byte_uri()));
            });
    }

    pub fn game_side(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // 刚开始时不显示原图
        if self.game_state().init {
            set_show_origin_image(false);
        }

        // 直接使用整个可用区域，不再添加额外边距
        // 这样可以充分利用右侧30%的列空间
        let full = ui.available_size();

        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                // 动态计算间距
                let space_unit = full.y * 0.05; // 使用窗口高度的5%作为基本间距单位

                ui.add_space(space_unit);

                // 动态计算字体大小
                let difficulty_font_size = f32::min(25.0, full.y * 0.04);

                match self.game_state().count {
                    3 => {
                        ui.label(egui::RichText::new("✨Easy").size(difficulty_font_size));
                    }
                    5 => {
                        ui.label(egui::RichText::new("🔥Normal").size(difficulty_font_size));
                    }
                    8 => {
                        ui.label(egui::RichText::new("💀Difficult").size(difficulty_font_size));
                    }
                    _ => {
                        ui.label(egui::RichText::new("🔧Custom").size(difficulty_font_size));
                    }
                }
                ui.add_space(space_unit);

                // ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(51,0,105);
                // ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(96,96,96);
                // ui.visuals_mut().widgets.active.weak_bg_fill = egui::Color32::from_rgb(96,96,96);

                // 动态计算按钮大小
                let button_width = f32::min(120.0, full.x * 0.8);
                let button_height = f32::min(40.0, full.y * 0.05);
                let button_font_size = f32::min(21.0, full.y * 0.035);

                let bot_resp = ui.add_sized(
                    [button_width, button_height],
                    Button::selectable(
                        self.game_state().bot,
                        egui::RichText::new("🎱 Bot").size(button_font_size),
                    ),
                );

                if bot_resp.clicked() {
                    match self.game_state().bot {
                        true => self.game_state().bot = false,
                        false => self.game_state().bot = true,
                    }
                }

                if self.game_state().bot {
                    ui.add_space(space_unit * 0.1);
                    let magic_font_size = f32::min(16.0, full.y * 0.025);
                    ui.label(
                        egui::RichText::new("🎉 Magic show")
                            .size(magic_font_size)
                            .color(egui::Color32::GOLD),
                    );
                }

                ui.add_space(space_unit);

                ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_BLUE;

                // 这里重开一个ui，不然按钮的长度会因为justified被强制拉长至和layout一样长
                ui.vertical_centered(|ui| {
                    let original_font_size = f32::min(17.0, full.y * 0.028);
                    let hover_image_size = f32::min(200.0, full.x * 1.5); // 悬停图片大小

                    let show_image_resp = ui
                        .add_sized(
                            [button_width, button_height],
                            egui::Button::new(
                                egui::RichText::new("Original").size(original_font_size),
                            ),
                        )
                        .on_hover_ui(|ui| {
                            ui.add_sized(
                                [hover_image_size, hover_image_size],
                                egui::Image::from_uri(self.img().get_byte_uri()),
                            );
                        });

                    if show_image_resp.clicked() {
                        if *SHOW_ORIGIN_IMAGE.lock().unwrap() {
                            set_show_origin_image(false);
                        } else {
                            set_show_origin_image(true);
                        }
                    }

                    if *SHOW_ORIGIN_IMAGE.lock().unwrap() {
                        self.show_origin_image(ctx, ui, &mut SHOW_ORIGIN_IMAGE.lock().unwrap());
                    }

                    ui.add_space(space_unit);

                    ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::RED;
                    let exit_font_size = f32::min(17.0, full.y * 0.028);
                    let return_resp = ui.add_sized(
                        [button_width, button_height],
                        egui::Button::new(egui::RichText::new("Exit").size(exit_font_size)),
                    );

                    if return_resp.clicked() {
                        self.ui_state().nav = super::Nav::Home;
                        self.game_state().reset_game_state();
                    }

                    let time_dispaly = match self.game_state().challenge {
                        true => self.get_rest_time_str(),
                        false => self.get_elasp_time_str(),
                    };

                    // 根据时间长度调整字体大小
                    let size_font = |l: f32| -> f32 {
                        if l <= 22.0 {
                            f32::min(25.0, full.y * 0.04)
                        } else {
                            let a = FONT_SIZE_BASE / (l / FONT_SIZE_DIVISOR);
                            let min_size = f32::min(12.0, full.y * 0.02);
                            if a > min_size {
                                a
                            } else {
                                min_size
                            }
                        }
                    };

                    // 挑战模式时时间为红色和绿色
                    // 非挑战模式时时间为蓝色
                    let time_color = if self.game_state().challenge
                        && self.game_state().rest < TimeDelta::seconds(21)
                    {
                        egui::Color32::LIGHT_RED
                    } else if self.game_state().challenge {
                        egui::Color32::LIGHT_GREEN
                    } else {
                        egui::Color32::LIGHT_BLUE
                    };

                    ui.add_space(space_unit * 0.6);
                    let time_font_size = f32::min(26.0, full.y * 0.042);
                    ui.label(
                        egui::RichText::new(time_dispaly)
                            .size(time_font_size)
                            .font(egui::FontId::monospace(size_font(5.0)))
                            .color(time_color),
                    );
                    // 请求重绘保证时间连续变化
                    ui.ctx().request_repaint();

                    if self.game_state().end && !self.game_state().win {
                        //ui.is_visible();
                        let fail_width = f32::min(80.0, full.x * 0.6);
                        let fail_height = f32::min(19.0, full.y * 0.03);
                        ui.add_sized(
                            [fail_width, fail_height],
                            egui::Label::new("You shall be better next time"),
                        );
                    }
                });
                // ui.add_space(60.0);
            },
        );
    }
}
