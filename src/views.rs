/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2025-07-14 15:41:08
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-22 19:18:10
 * @FilePath: /jigsaw-puzzle-rust/src/views.rs
 * @Description:
 *
 * Copyright (c) 2025 by goodpeanuts, All Rights Reserved.
 */
use crate::app::{AppError, GameApp};
use eframe::egui;
use std::sync::{Mutex, OnceLock};

pub mod custom_widget;
pub mod home;
pub mod playground;
pub mod sidebar;

const BUTTON_FONT_SIZE_MAX: f32 = 20.0;
const BUTTON_FONT_SIZE_RATIO: f32 = 0.14;
const BUTTON_SIZE_RATIO: f32 = 0.6;

//
//

#[derive(Clone, Copy)]
pub enum Nav {
    Home,
    Game,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DisplayDirection {
    Vertical,
    Horizontal,
    Unknown,
}

impl DisplayDirection {
    const ASPECT_RATIO_THRESHOLD: f32 = 0.88;

    fn get_direction(width: f32, height: f32) -> Self {
        if height * Self::ASPECT_RATIO_THRESHOLD > width {
            Self::Vertical
        } else {
            Self::Horizontal
        }
    }
}

pub static GLOBAL_UI_STATE: OnceLock<Mutex<GlobalUIState>> = OnceLock::new();

pub fn update_global_ui_state(ctx: &egui::Context) -> anyhow::Result<()> {
    let global = GLOBAL_UI_STATE
        .get()
        .ok_or_else(|| anyhow::anyhow!("GLOBAL_UI_STATE not initialized"))?;
    let mut guard = global
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock GLOBAL_UI_STATE: {}", e))?;
    guard.update(ctx);
    Ok(())
}

pub fn update_global_ui_state_with_error_info(ctx: &egui::Context, ui: &mut egui::Ui) {
    let _ = update_global_ui_state(ctx)
        .inspect_err(|e| AppError::ShowInfoLable(e.to_string()).show(ui));
}

pub fn get_global_ui_direction() -> DisplayDirection {
    let global = if let Some(g) = GLOBAL_UI_STATE.get() {
        g
    } else {
        return DisplayDirection::Unknown;
    };

    if let Ok(guard) = global.lock() {
        guard.direction()
    } else {
        DisplayDirection::Unknown
    }
}

pub struct GlobalUIState {
    direction: DisplayDirection,
}

impl GlobalUIState {
    pub fn new(cc: &egui::Context) -> Self {
        let screen_rect = cc.screen_rect();
        let width = screen_rect.width();
        let height = screen_rect.height();
        GlobalUIState {
            direction: DisplayDirection::get_direction(width, height),
        }
    }

    pub(crate) fn update(&mut self, ctx: &egui::Context) {
        let screen_rect = ctx.screen_rect();
        let width = screen_rect.width();
        let height = screen_rect.height();
        self.direction = DisplayDirection::get_direction(width, height);
    }

    pub(crate) fn direction(&self) -> DisplayDirection {
        self.direction
    }
}

lazy_static::lazy_static! {
    pub static ref SHOW_ORIGIN_IMAGE: Mutex<bool> = Mutex::new(false);
}

pub fn set_show_origin_image(value: bool) {
    let mut show_origin_image = SHOW_ORIGIN_IMAGE.lock().unwrap();
    *show_origin_image = value;
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.ui_state().nav {
            Nav::Home => {
                update_global_ui_state_with_error_info(ctx, ui);
                self.home(ctx, ui);
            }
            Nav::Game => {
                update_global_ui_state_with_error_info(ctx, ui);
                self.game(ctx, ui);
                if !self.game_state().end && self.game_state().bot {
                    self.recover();
                }
                if self.game_state().win {
                    self.congratulation(
                        ctx,
                        ui,
                        &mut playground::SHOW_CONGRULATION.lock().unwrap(),
                    );
                }
                // 刚开始时不显示原图
                if self.game_state().init {
                    set_show_origin_image(false);
                }
            }
        });
    }
}
