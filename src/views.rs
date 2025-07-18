use eframe::egui;

use crate::app::{state, GameApp};

/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2025-07-14 15:41:08
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-18 14:09:41
 * @FilePath: /jigsaw-puzzle-rust/src/views.rs
 * @Description:
 *
 * Copyright (c) 2025 by goodpeanuts, All Rights Reserved.
 */
pub mod custom_widget;
pub mod home;
pub mod playground;
pub mod sidebar;

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.ui_state().nav {
            state::Nav::Home => {
                self.home(ctx, ui);
            }
            state::Nav::Game => {
                self.playground(ctx, ui);
                self.game_side(ctx, ui);
                if !self.game_state().end && self.game_state().bot {
                    self.recover();
                }
            }
        });
    }
}
