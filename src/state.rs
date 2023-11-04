/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-03 19:28:31
 * @FilePath: \puzzle\src\state.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use eframe::egui;
use egui::{Image, Ui};

#[derive(Default)]
pub struct GameState {
    pub start: bool,
    pub img: String,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            start: false,
            img: "../assets/img/1.png".to_string(),
        }
    }
}