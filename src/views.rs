/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2025-07-14 15:41:08
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-21 14:58:58
 * @FilePath: /jigsaw-puzzle-rust/src/views.rs
 * @Description:
 *
 * Copyright (c) 2025 by goodpeanuts, All Rights Reserved.
 */
use crate::app::GameApp;
use eframe::egui;

pub mod custom_widget;
pub mod home;
pub mod playground;
pub mod sidebar;

#[derive(Clone, Copy)]
pub enum Nav {
    Home,
    Game,
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.ui_state().nav {
            Nav::Home => {
                self.home(ctx, ui);
            }
            Nav::Game => {
                self.game(ctx, ui);
                if !self.game_state().end && self.game_state().bot {
                    self.recover();
                }
            }
        });
    }
}
