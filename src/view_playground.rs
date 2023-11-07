/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-05 22:15:38
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-08 00:22:03
 * @FilePath: \puzzle\src\view_playground.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::game::GameApp;
use eframe::{
    egui::{self, Button},
    epaint::vec2,
};
use std::io;

use crate::state;
use std::io::Write;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref SHOW_CONGRULATION: Mutex<bool> = Mutex::new(false);
}

pub fn set_show_congrulation(value: bool) {
    let mut show_congrulation = SHOW_CONGRULATION.lock().unwrap();
    *show_congrulation = value;
}

impl GameApp {
    // 对局开始初始化，设置碎片个数并打乱，设置计时模式
    pub fn game_init(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // 如果是挑战模式，设置时间限制
        if self.game_state.challenge {
            match self.game_state.count {
                2 => {
                    self.game_state.limit = 10.0;
                }
                3 => {
                    self.game_state.limit = 15.0;
                }
                5 => {
                    self.game_state.limit = 90.0;
                }
                8 => {
                    self.game_state.limit = 480.0;
                }
                _ => {
                    self.game_state.limit = self.calculate_time_limit(self.game_state.count as f64);
                }
            }
        }
        /********** 用于调试 ************/
        print!("1 set game_state.limit success\n");
        self.game_state.start = std::time::Instant::now();
        /********** 用于调试 ************/
        print!("2 set game_state.start success\n");
        self.shuffle_pieces();
        /********** 用于调试 ************/
        print!("3 shuffle_pieces success\n");
        self.split_image(ctx);
        /********** 用于调试 ************/
        print!("4 split_image success\n");

        // 防止有人设置碎片为 1*1 , (*/ω＼*)
        self.check_game();

        /********** 用于调试 ************/
        print!("5 init success\n");

        /********** 用于调试 ************/
        print!("init flase\n");
        self.game_state.init = false;
    }

    pub fn playground(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // 计算大框矩形的位置
        let img_size = vec2(
            840.0 / self.game_state.count as f32,
            840.0 / self.game_state.count as f32,
        );
        let mut gap = 3.0; // 定义间隙宽度
        let mut offset = 15.0;
        let mut rect_stroke = 6.0;

        if self.game_state.count < 13 {
            offset = 15.0;
        } else if self.game_state.count < 29 {
            offset = 10.0;
            rect_stroke = 5.0;
            gap = 1.5;
        } else if self.game_state.count < 51 {
            offset = 5.0;
            rect_stroke = 3.0;
            gap = 1.0;
        } else {
            offset = 2.0;
            gap = 0.1;
            rect_stroke = 1.0;
        }

        let offset_pos = egui::pos2(offset, offset); // 定义起点偏移量
        let total_gap = gap * (self.game_state.count as f32 - 1.0); // 总的间隙宽度
        let total_size = img_size.x * self.game_state.count as f32 + total_gap; // 总的大小
        let big_rect = egui::Rect::from_min_max(
            egui::pos2(offset - gap, offset - gap), // 减去边缘的偏移量
            egui::pos2(offset + total_size + gap, offset + total_size + gap), // 加上边缘的偏移量
        );
        ui.painter()
            .rect_stroke(big_rect, 0.0, egui::Stroke::new(rect_stroke, egui::Color32::WHITE));
        // --------------- diaplay image
        for i in 0..self.game_state.count {
            ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 1.0);
            ui.horizontal(|ui| {
                for j in 0..self.game_state.count {
                    let pos = i * self.game_state.count + j; // 界面中碎片的位置

                    // 将init设置为false的时机放在 game_init 的最后一步，否则这里就会因为重开时没有初始化访问到空数组
                    let index = self.game_state.pos[pos as usize] as usize;

                    /********** 用于调试 ************/
                    if self.game_state.init {
                        print!("[{:>2} - {:>2}] ", pos, index);
                    }

                    // 计算拼图碎片的位置
                    let rect = egui::Rect::from_min_max(
                        egui::pos2(
                            j as f32 * (img_size.x + gap) + offset_pos.x,
                            i as f32 * (img_size.y + gap) + offset_pos.y,
                        ),
                        egui::pos2(
                            (j + 1) as f32 * (img_size.x + gap) + offset_pos.x,
                            (i + 1) as f32 * (img_size.y + gap) + offset_pos.y,
                        ),
                    );

                    ui.allocate_ui_at_rect(rect, |ui| {
                        ui.allocate_ui_at_rect(rect, |ui| {
                            let response = ui
                                .add_sized(
                                    [
                                        840.0 / self.game_state.count as f32,
                                        840.0 / self.game_state.count as f32,
                                    ],
                                    egui::Image::from_uri(
                                        self.game_state.pieces[index as usize].uri.clone(),
                                    ),
                                )
                                .interact(egui::Sense::click());

                            if !self.game_state.bot && response.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            };

                            if !self.game_state.bot && response.clicked() {
                                /********** 用于调试 ************/
                                // 打印点击的位置
                                print!("{} ", pos);
                                io::stdout().flush().unwrap();
                                self.game_state.exchange.push(pos);
                                ui.ctx().request_repaint();
                                self.exchange_piece();
                                ui.ctx().request_repaint();
                            }

                            if self.game_state.exchange.contains(&pos) {
                                // ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(255, 0, 0));
                                let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
                                ui.painter().rect_stroke(response.rect, 0.0, stroke);
                            }
                        });
                    });
                }
                if self.game_state.init {
                    println!();
                }
            });
        }
        if self.game_state.win {
            self.congratulation(ctx, ui, &mut *SHOW_CONGRULATION.lock().unwrap());
        }
    }

    fn congratulation(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, is_open: &mut bool) {
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
                        egui::Image::from_uri(self.img.get_byte_uri()),
                    );
                });

                let restart_resp = ui
                    .add_sized([100.0, 30.0], Button::new("Again"))
                    .on_hover_text("Back to menu")
                    .clicked();

                if restart_resp {
                    self.ui_state.nav = state::Nav::Home;
                    self.game_state.reset_game_state();
                }
            });
    }

}
