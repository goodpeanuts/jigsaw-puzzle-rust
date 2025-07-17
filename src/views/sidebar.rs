/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-07 10:31:27
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-09 01:25:40
 * @FilePath: \puzzle\src\view_gameside.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */

use crate::game::GameApp;
use crate::state;
use eframe::egui::{self, Button, UiBuilder};
use std::sync::Mutex;

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
                ui.add(egui::Image::from_uri(self.get_img().get_byte_uri()));
            });
    }

    pub fn game_side(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // 刚开始时不显示原图
        if self.get_game_state().init {
            set_show_origin_image(false);
        }

        let game_side_rect =
            egui::Rect::from_min_max(egui::pos2(900.0, 20.0), egui::pos2(1200.0, 900.0));
        ui.scope_builder(UiBuilder::new().max_rect(game_side_rect), |ui| {
            ui.allocate_ui_with_layout(
                ui.available_size(),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    // 画一个黄色填充的正方形
                    // ui.painter().rect_filled(game_side_rect, 0.0, egui::Color32::from_rgb(255, 255, 0));
                    // ui.spacing_mut().item_spacing = egui::Vec2::new(20.0, 20.0);
                    ui.add_space(50.0);
                    match self.get_game_state().count {
                        3 => {
                            ui.label(egui::RichText::new("✨Easy").size(25.0));
                        }
                        5 => {
                            ui.label(egui::RichText::new("🔥Normal").size(25.0));
                        }
                        8 => {
                            ui.label(egui::RichText::new("💀Difficult").size(25.0));
                        }
                        _ => {
                            ui.label(egui::RichText::new("🔧Custom").size(25.0));
                        }
                    }
                    ui.add_space(50.0);

                    // ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(51,0,105);
                    // ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(96,96,96);
                    // ui.visuals_mut().widgets.active.weak_bg_fill = egui::Color32::from_rgb(96,96,96);

                    let bot_resp = ui.add_sized(
                        [120.0, 40.0],
                        Button::selectable(
                            self.get_game_state().bot,
                            egui::RichText::new("🎱 Bot").size(21.0),
                        ),
                    );

                    if bot_resp.clicked() {
                        match self.get_game_state().bot {
                            true => self.get_game_state().bot = false,
                            false => self.get_game_state().bot = true,
                        }
                    }

                    if self.get_game_state().bot {
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new("🎉 Magic show")
                                .size(16.0)
                                .color(egui::Color32::GOLD),
                        );
                    }

                    ui.add_space(50.0);

                    ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_BLUE;

                    // 这里重开一个ui，不然按钮的长度会因为justified被强制拉长至和layout一样长
                    ui.vertical_centered(|ui| {
                        let show_imgea_resp = ui
                            .add_sized(
                                [120.0, 40.0],
                                egui::Button::new(egui::RichText::new("Original").size(17.0)),
                            )
                            .on_hover_ui(|ui| {
                                ui.add_sized(
                                    [200.0, 200.0],
                                    egui::Image::from_uri(self.get_img().get_byte_uri()),
                                );
                            });

                        if show_imgea_resp.clicked() {
                            if *SHOW_ORIGIN_IMAGE.lock().unwrap() {
                                set_show_origin_image(false);
                            } else {
                                set_show_origin_image(true);
                            }
                        }

                        if *SHOW_ORIGIN_IMAGE.lock().unwrap() {
                            self.show_origin_image(ctx, ui, &mut SHOW_ORIGIN_IMAGE.lock().unwrap());
                        }

                        ui.add_space(50.0);

                        ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::RED;
                        let return_resp = ui.add_sized(
                            [120.0, 40.0],
                            egui::Button::new(egui::RichText::new("Exit").size(17.0)),
                        );

                        if return_resp.clicked() {
                            self.get_ui_state().nav = state::Nav::Home;
                            self.get_game_state().reset_game_state();
                        }

                        let time_dispaly = match self.get_game_state().challenge {
                            true => self.get_rest_time_str(),
                            false => self.get_elasp_time_str(),
                        };

                        // 根据时间长度调整字体大小
                        let size_font = |l: f32| -> f32 {
                            if l <= 22.0 {
                                25.0
                            } else {
                                let a = 330.0 / (l / 0.6);
                                if a > 12.0 {
                                    a
                                } else {
                                    12.0
                                }
                            }
                        };

                        // 挑战模式时时间为红色和绿色
                        // 非挑战模式时时间为蓝色
                        let time_color = if self.get_game_state().challenge
                            && self.get_game_state().rest < 21.0
                        {
                            egui::Color32::LIGHT_RED
                        } else if self.get_game_state().challenge {
                            egui::Color32::LIGHT_GREEN
                        } else {
                            egui::Color32::LIGHT_BLUE
                        };

                        ui.add_space(30.0);
                        ui.label(
                            egui::RichText::new(time_dispaly)
                                .size(26.0)
                                .font(egui::FontId::monospace(size_font(5.0)))
                                .color(time_color),
                        );
                        // 请求重绘保证时间连续变化
                        ui.ctx().request_repaint();

                        if self.get_game_state().end && !self.get_game_state().win {
                            //ui.is_visible();
                            ui.add_sized(
                                [80.0, 19.0],
                                egui::Label::new("You shall be better next time"),
                            );
                        }
                    });
                    // ui.add_space(60.0);
                },
            )
        });
    }
}
