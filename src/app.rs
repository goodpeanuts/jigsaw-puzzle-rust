/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-18 17:57:34
 * @FilePath: /jigsaw-puzzle-rust/src/app.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use anyhow;
use wasm_bindgen::prelude::*;

use crate::common::images;

pub(crate) mod game;
pub(crate) mod state;

#[wasm_bindgen]
#[derive(Clone)]
pub struct GameApp {
    ui_state: state::UiState,
    game_state: state::GameState,
    img: images::ImageChoice,
}

impl GameApp {
    pub fn img(&self) -> &images::ImageChoice {
        &self.img
    }

    pub fn ui_state(&mut self) -> &mut state::UiState {
        &mut self.ui_state
    }

    pub fn game_state(&mut self) -> &mut state::GameState {
        &mut self.game_state
    }

    pub fn mut_img(&mut self) -> &mut images::ImageChoice {
        &mut self.img
    }

    pub fn new(_cc: &eframe::CreationContext<'_>) -> anyhow::Result<Self> {
        #[cfg(feature = "chinese")]
        crate::common::config::custom_font(_cc);

        let app = std::panic::catch_unwind(|| GameApp {
            game_state: state::GameState::new(),
            ui_state: state::UiState {
                nav: crate::views::Nav::Home,
            },
            img: images::ImageChoice::Image6,
        })
        .map_err(|_| anyhow::anyhow!("Failed to initialize game application"))?;
        Ok(app)
    }
}
