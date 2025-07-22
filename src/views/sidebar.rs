/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-07 10:31:27
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-22 19:18:15
 * @FilePath: /jigsaw-puzzle-rust/src/views/sidebar.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */

use crate::{app::GameApp, common::time::TimeDelta};
use eframe::egui::{self, Button, UiBuilder};
use egui_extras::{Size, StripBuilder};

use super::{
    get_global_ui_direction, set_show_origin_image, DisplayDirection, BUTTON_FONT_SIZE_MAX,
    BUTTON_FONT_SIZE_RATIO, BUTTON_SIZE_RATIO, SHOW_ORIGIN_IMAGE,
};

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
        ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_BLUE;

        let direction = get_global_ui_direction();
        match direction {
            DisplayDirection::Vertical => {
                ui.scope_builder(UiBuilder::new(), |ui| {
                    StripBuilder::new(ui)
                        .size(Size::relative(0.4))
                        .size(Size::relative(0.2))
                        .size(Size::relative(0.2))
                        .size(Size::relative(0.2))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.vertical_centered_justified(|ui| {
                                    self.difficulty_title(ctx, ui);
                                    self.time(ctx, ui);
                                });
                            });

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    self.bot_button(ctx, ui);
                                });
                            });

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    self.origin_button(ctx, ui);
                                });
                            });

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    self.exit_button(ctx, ui);
                                });
                            });
                        });
                });
            }
            _ => {
                ui.scope_builder(UiBuilder::new(), |ui| {
                    StripBuilder::new(ui)
                        .size(Size::relative(0.2))
                        .size(Size::relative(0.2))
                        .size(Size::relative(0.2))
                        .size(Size::relative(0.2))
                        .size(Size::relative(0.2))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    self.difficulty_title(ctx, ui);
                                });
                            });

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    self.time(ctx, ui);
                                });
                            });

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    self.bot_button(ctx, ui);
                                });
                            });

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    self.origin_button(ctx, ui);
                                });
                            });

                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    self.exit_button(ctx, ui);
                                });
                            });
                        });
                });
            }
        }
    }

    fn difficulty_title(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let available = f32::min(ui.available_size().y, ui.available_size().x);
        let font_size = available * 0.20;

        match self.game_state().count {
            3 => {
                ui.label(egui::RichText::new("✨Easy").size(font_size));
            }
            5 => {
                ui.label(egui::RichText::new("🔥Normal").size(font_size));
            }
            8 => {
                ui.label(egui::RichText::new("💀Difficult").size(font_size));
            }
            _ => {
                ui.label(egui::RichText::new("🔧Custom").size(font_size));
            }
        }
    }

    fn bot_button(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let available = f32::min(ui.available_size().y, ui.available_size().x);
        let button_font_size = f32::min(BUTTON_FONT_SIZE_MAX, available * BUTTON_FONT_SIZE_RATIO);
        let button_size = available * BUTTON_SIZE_RATIO; // 改为和其他按钮一样的大小

        let bot_resp = ui.add_sized(
            [button_size, button_size * 0.4],
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
    }

    fn origin_button(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let available = f32::min(ui.available_size().y, ui.available_size().x);
        let button_font_size = f32::min(BUTTON_FONT_SIZE_MAX, available * BUTTON_FONT_SIZE_RATIO);
        let button_size = available * BUTTON_SIZE_RATIO; // 改为和其他按钮一样的大小

        let hover_image_size = f32::min(200.0, available * 1.5); // 悬停图片大小

        let show_image_resp = ui
            .add_sized(
                [button_size, button_size * 0.4],
                egui::Button::new(egui::RichText::new("Original").size(button_font_size)),
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
    }

    fn exit_button(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let available = f32::min(ui.available_size().y, ui.available_size().x);
        let button_font_size = f32::min(BUTTON_FONT_SIZE_MAX, available * BUTTON_FONT_SIZE_RATIO);
        let button_size = available * BUTTON_SIZE_RATIO; // 改为和其他按钮一样的大小

        ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::RED;
        let return_resp = ui.add_sized(
            [button_size, button_size * 0.4],
            egui::Button::new(egui::RichText::new("Exit").size(button_font_size)),
        );

        if return_resp.clicked() {
            self.ui_state().nav = super::Nav::Home;
            self.game_state().reset_game_state();
        }
    }

    fn time(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let available = f32::min(ui.available_size().y, ui.available_size().x);
        let font_size = available * 0.26;

        let time_dispaly = match self.game_state().challenge {
            true => self.get_rest_time_str(),
            false => self.get_elasp_time_str(),
        };

        // 挑战模式时时间为红色和绿色
        // 非挑战模式时时间为蓝色
        let time_color =
            if self.game_state().challenge && self.game_state().rest < TimeDelta::seconds(21) {
                egui::Color32::LIGHT_RED
            } else if self.game_state().challenge {
                egui::Color32::LIGHT_GREEN
            } else {
                egui::Color32::LIGHT_BLUE
            };

        ui.label(
            egui::RichText::new(time_dispaly)
                .size(font_size)
                .font(egui::FontId::monospace(font_size))
                .color(time_color),
        );
        // 请求重绘保证时间连续变化
        ui.ctx().request_repaint();

        if self.game_state().end && !self.game_state().win {
            let fail_width = f32::min(80.0, available * BUTTON_SIZE_RATIO);
            let fail_height = f32::min(19.0, available * 0.03);
            ui.add_sized(
                [fail_width, fail_height],
                egui::Label::new("You shall be better next time"),
            );
        }
    }
}
