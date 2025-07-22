/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-05 22:15:38
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-22 10:18:04
 * @FilePath: /jigsaw-puzzle-rust/src/views/playground.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::{app::GameApp, common::time::TimeDelta};
use eframe::{
    egui::{self, Button, CentralPanel, Pos2, UiBuilder},
    epaint::vec2,
};
use egui_extras::{Size, StripBuilder};

use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref SHOW_CONGRULATION: Mutex<bool> = Mutex::new(false);
}

pub fn set_show_congrulation(value: bool) {
    let mut show_congrulation = SHOW_CONGRULATION.lock().unwrap();
    *show_congrulation = value;
}

impl GameApp {
    pub(crate) fn game(&mut self, ctx: &egui::Context, _ui: &mut egui::Ui) {
        CentralPanel::default().show(ctx, |ui| {
            let full = ui.available_size();
            let margin_x = full.x * 0.06;
            let margin_y = full.y * 0.06;
            let margin = f32::min(margin_x, margin_y);
            let full_rect = egui::Rect::from_min_size(Pos2::new(0.0, 0.0), full);
            let margin_rect =
                egui::Rect::from_min_max(full_rect.min + vec2(margin, margin), full_rect.max);

            ui.scope_builder(UiBuilder::new().max_rect(margin_rect), |ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.85))
                    .size(Size::relative(0.15))
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            egui::Frame::new()
                                .fill(egui::Color32::from_additive_luminance(8))
                                .show(ui, |ui| {
                                    self.puzzle(ctx, ui);
                                });
                        });
                        strip.cell(|ui| {
                            egui::Frame::new()
                                .fill(egui::Color32::from_additive_luminance(8))
                                .show(ui, |ui| {
                                    ui.centered_and_justified(|ui| {
                                        self.game_side(ctx, ui);
                                    });
                                });
                        });
                    });
            })
        });
    }

    fn puzzle(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let count = self.game_state().count;
        let full = ui.available_size();
        let square_len = f32::min(full.x, full.y);
        let offset = square_len * 0.02;
        let offset_pos = egui::pos2(offset, offset);
        let square_stroke_width = square_len * 0.008;
        let square_vec2 = egui::Vec2::new(square_len, square_len);
        let square_rect = egui::Rect::from_min_size(offset_pos, square_vec2);

        // puzzle square
        if self.game_state().challenge
            && self.game_state().start.elapsed() < TimeDelta::seconds(21.0)
        {
            ui.painter().rect_stroke(
                square_rect,
                0.0,
                egui::Stroke::new(square_stroke_width, egui::Color32::LIGHT_RED),
                egui::StrokeKind::Outside,
            );
        } else if self.game_state().challenge
            && self.game_state().start.elapsed() >= TimeDelta::seconds(21.0)
        {
            ui.painter().rect_stroke(
                square_rect,
                0.0,
                egui::Stroke::new(square_stroke_width, egui::Color32::from_rgb(178, 102, 255)),
                egui::StrokeKind::Outside,
            );
        } else {
            ui.painter().rect_stroke(
                square_rect,
                0.0,
                egui::Stroke::new(square_stroke_width, egui::Color32::LIGHT_BLUE),
                egui::StrokeKind::Outside,
            );
        }

        let piece_len = square_len / count as f32;
        let gap = piece_len * 0.005;
        let piece_image_len = piece_len - gap * 2.0;

        // display pieces image
        for i in 0..count {
            ui.horizontal(|ui| {
                for j in 0..count {
                    let pos = i * count + j; // 界面中碎片的位置

                    // 将init设置为false的时机放在 game_init 的最后一步，否则这里就会因为重开时没有初始化访问到空数组
                    let index = self.game_state().pos[pos as usize] as usize;

                    #[cfg(feature = "debug")]
                    if self.game_state().init {
                        println!("[{:>2} - {:>2}]", pos, index);
                    }

                    // 计算拼图碎片的位置
                    let rect = egui::Rect::from_min_max(
                        egui::pos2(
                            j as f32 * piece_len + gap + offset,
                            i as f32 * piece_len + gap + offset,
                        ),
                        egui::pos2(
                            (j + 1) as f32 * piece_len + gap + offset,
                            (i + 1) as f32 * piece_len + gap + offset,
                        ),
                    );

                    ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                        ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                            let response = ui
                                .add_sized(
                                    [piece_image_len, piece_image_len],
                                    egui::Image::from_uri(
                                        self.game_state().pieces[index].uri.clone(),
                                    ),
                                )
                                .interact(egui::Sense::click());

                            if !self.game_state().bot && response.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            };

                            if !self.game_state().bot && response.clicked() {
                                #[cfg(feature = "debug")]
                                {
                                    print!("{} ", pos);
                                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                                }
                                self.game_state().exchange.push(pos);
                                ui.ctx().request_repaint();
                                self.exchange_piece();
                                ui.ctx().request_repaint();
                            }

                            if self.game_state().exchange.contains(&pos) {
                                let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
                                ui.painter().rect_stroke(
                                    response.rect,
                                    0.0,
                                    stroke,
                                    egui::StrokeKind::Middle,
                                );
                            }
                        });
                    });
                }
                if self.game_state().init {
                    println!();
                }
            });
        }
        if self.game_state().win {
            self.congratulation(ctx, ui, &mut SHOW_CONGRULATION.lock().unwrap());
        }
    }

    fn congratulation(&mut self, ctx: &egui::Context, _ui: &mut egui::Ui, is_open: &mut bool) {
        egui::Window::new("💕Congratulations")
            .title_bar(true)
            .fixed_pos(egui::pos2(0.0, 0.0))
            .open(is_open)
            .fixed_size([900.0, 900.0])
            .default_open(true)
            .collapsible(false)
            .default_size(egui::vec2(900.0, 900.0))
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label(egui::RichText::new("YOU MADE IT！").size(20.0));
                    ui.add_sized(
                        [800.0, 800.0],
                        egui::Image::from_uri(self.img().get_byte_uri()),
                    );
                });

                let restart_resp = ui
                    .add_sized([100.0, 30.0], Button::new("Again"))
                    .on_hover_text("Back to menu")
                    .clicked();

                if restart_resp {
                    self.ui_state().nav = super::Nav::Home;
                    self.game_state().reset_game_state();
                }
            });
    }
}
