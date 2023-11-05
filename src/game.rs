/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-06 00:47:27
 * @FilePath: \puzzle\src\game.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use eframe::{egui::{self, Button}, epaint::vec2};
use image::GenericImageView;
use std::io::Cursor;
use rand::prelude::SliceRandom;

use crate::{config, state, imgs};

pub struct GameApp {
    pub ui_state: state::UiState,
    pub game_state: state::GameState,
    pub img: imgs::ImageChoice,
}

impl GameApp {

    // 将Vec<u8>转换为&'static [u8], 确保其拼图碎片拥有和程序一样长的生命周期
    fn get_static_u8(bytes: &Vec<u8>) -> &'static [u8] {
        let x = bytes.clone().into_boxed_slice();
        let static_ref = Box::leak(x);
        &static_ref[..]
    }

    pub fn split_image(&mut self, cc: &egui::Context) {
        let mut img = image::load_from_memory(imgs::ImageChoice::chose_pic(&self.img)).expect("Failed to load image");
        let (width, height) = img.dimensions();
        let sub_width = width / self.game_state.count as u32;
        let sub_height = height / self.game_state.count as u32;
        for i in 0..self.game_state.count {
            for j in 0..self.game_state.count {
                let x = sub_width * j as u32;
                let y = sub_height * i as u32;
                //print!("{}", i * self.game_state.count + j);
                // 从原图中裁剪出子图
                let sub_image = img.crop(x, y, sub_width, sub_height);

                // 将处理好的图片写入到缓存中
                let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
                sub_image.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
                let bytes = buf.into_inner();

                // 将缓存中的图片写入到egui的缓存中
                cc.include_bytes(format!("bytes://{}", i * self.game_state.count + j), GameApp::get_static_u8(&bytes));
            }
        }
    }

    // 初始化游戏，打乱碎片
    pub fn init_game(&mut self) {
        for i in 0..self.game_state.count * self.game_state.count {
            self.game_state.pos.push(i);
        }
        self.game_state.pos.shuffle(&mut rand::thread_rng());
    }

    pub fn exchange_piece(&mut self) {
        if self.game_state.exchange.len() == 2 {
            /********** 用于调试 ************/
            print!("[{:>2} - {:>2}]  exchanged\n", self.game_state.exchange[0], self.game_state.exchange[1]);
            self.game_state.pos.swap(self.game_state.exchange[0].try_into().unwrap(), self.game_state.exchange[1].try_into().unwrap());
            self.game_state.exchange.clear();
        }
        self.check_win();
    }

    fn check_win(&mut self) {
        let mut flag = true;
        for i in 0..self.game_state.count * self.game_state.count {
            if self.game_state.pos[i as usize] != i {
                flag = false;
                break;
            }
        }
        if flag {
            self.game_state.win = true;
            println!("You win!");
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        config::custom_font(cc);
        GameApp {
            game_state: state::GameState::new(),
            ui_state: state::UiState {
                nav: state::Nav::Home,
            },
            img: imgs::ImageChoice::Image6,
        }
    }


}   

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {  
        egui::CentralPanel::default().show(ctx, 
            |ui|{
                match self.ui_state.nav {
                    state::Nav::Home => {
                        self.home(ctx, ui);
                    },
                    state::Nav::Game => {
                        self.playground(ctx, ui);
                    }
                }
            });
    }
}