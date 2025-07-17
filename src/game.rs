/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-17 20:27:14
 * @FilePath: /jigsaw-puzzle-rust/src/game.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use anyhow;
use eframe::egui;
use image::GenericImageView;
use rand::prelude::SliceRandom;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

use crate::{
    imgs, state,
    time::{TimeDelta, TimeStamp},
    views::playground,
};

#[wasm_bindgen]
#[derive(Clone)]
pub struct GameApp {
    ui_state: state::UiState,
    game_state: state::GameState,
    #[wasm_bindgen(getter)]
    img: imgs::ImageChoice,
}

use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
impl GameApp {
    #[wasm_bindgen(getter)]
    pub fn img(&self) -> JsValue {
        to_value(&self.img).unwrap()
    }
}

impl GameApp {
    pub fn get_img(&self) -> &imgs::ImageChoice {
        &self.img
    }

    pub fn get_ui_state(&mut self) -> &mut state::UiState {
        &mut self.ui_state
    }

    pub fn get_game_state(&mut self) -> &mut state::GameState {
        &mut self.game_state
    }

    pub fn get_mut_img(&mut self) -> &mut imgs::ImageChoice {
        &mut self.img
    }
}

impl GameApp {
    // 将Vec<u8>转换为&'static [u8], 确保其拼图碎片拥有和程序一样长的生命周期
    fn get_static_u8(bytes: &[u8]) -> &'static [u8] {
        let x = bytes.to_owned().into_boxed_slice();
        let static_ref = Box::leak(x);
        &static_ref[..]
    }

    pub fn split_image(&mut self, cc: &egui::Context) {
        self.game_state.create_pieces_index();
        let mut img = image::load_from_memory(imgs::ImageChoice::chose_pic(&self.img))
            .expect("Failed to load image");
        let (width, height) = img.dimensions();
        let sub_width = width / self.game_state.count;
        let sub_height = height / self.game_state.count;
        for i in 0..self.game_state.count {
            for j in 0..self.game_state.count {
                let x = sub_width * j;
                let y = sub_height * i;
                // 从原图中裁剪出子图
                let sub_image = img.crop(x, y, sub_width, sub_height);

                // 将处理好的图片写入到缓存中
                let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
                sub_image
                    .write_to(&mut buf, image::ImageFormat::Png)
                    .unwrap();
                let bytes = buf.into_inner();

                // 将缓存中的图片写入到egui的缓存中
                cc.include_bytes(
                    // !
                    format!(
                        "bytes://{}",
                        self.game_state.index_offset + i * self.game_state.count + j
                    ),
                    GameApp::get_static_u8(&bytes),
                );
            }
        }
    }

    // 初始化游戏，打乱碎片索引与位置的对应关系
    pub fn shuffle_pieces(&mut self) {
        for i in 0..self.game_state.count * self.game_state.count {
            self.game_state.pos.push(i);
        }
        self.game_state.pos.shuffle(&mut rand::rng());
    }

    pub fn exchange_piece(&mut self) {
        if self.game_state.exchange.len() == 2 {
            #[cfg(feature = "debug")]
            println!(
                "[{:>2} - {:>2}]  exchanged",
                self.game_state.exchange[0], self.game_state.exchange[1]
            );
            self.game_state.pos.swap(
                self.game_state.exchange[0].try_into().unwrap(),
                self.game_state.exchange[1].try_into().unwrap(),
            );
            self.game_state.exchange.clear();
        }
        self.check_game();
    }

    pub fn get_recovery(&mut self) {
        let mut index = 0;
        self.game_state.recovery = vec![
            0;
            (self.game_state.count * self.game_state.count)
                .try_into()
                .unwrap()
        ];
        for layer in 0..(self.game_state.count + 1).div_ceil(2) {
            let start = layer;
            let end = self.game_state.count - layer;
            // Traverse right
            for i in start..end {
                self.game_state.recovery[index] = layer * self.game_state.count + i;
                index += 1;
            }
            // Traverse down
            for i in start + 1..end {
                self.game_state.recovery[index] = i * self.game_state.count + end - 1;
                index += 1;
            }
            // Traverse left
            for i in (start..end - 1).rev() {
                self.game_state.recovery[index] = (end - 1) * self.game_state.count + i;
                index += 1;
            }
            // Traverse up
            for i in (start + 1..end - 1).rev() {
                self.game_state.recovery[index] = i * self.game_state.count + start;
                index += 1;
            }
        }
        // 将vector中的元素反转
        self.game_state.recovery.reverse();

        #[cfg(feature = "debug")]
        {
            println!("");
            for i in 0..self.game_state.count * self.game_state.count {
                print!("{} ", self.game_state.recovery[i as usize]);
                if (i + 1) % self.game_state.count == 0 {
                    println!("");
                }
            }
            println!("");
        }
    }

    // 机器人操作复原
    pub fn recover(&mut self) {
        //控制复原速度
        if self.game_state.last_step_timestamp.elapsed() > TimeDelta::milliseconds(50) {
            self.game_state.last_step_timestamp = TimeStamp::instant();
        } else {
            return;
        }

        let pos_len = self.game_state.count * self.game_state.count;
        // 冒泡排序
        // for i in 0..pos_len {
        //     for j in 0..pos_len - i - 1 {
        //         if self.game_state.pos[j as usize] > self.game_state.pos[(j + 1) as usize] {
        //             self.game_state.exchange.push(j);
        //             self.game_state.exchange.push(j + 1);
        //             self.exchange_piece();
        //             return;
        //         }
        //     }
        // }

        // 第二种恢复，简单醋泡
        // for i in 0..pos_len {
        //     if self.game_state.pos[i as usize] != i {
        //         self.game_state.exchange.push(i);
        //         self.game_state.exchange.push(self.game_state.pos[i as usize] as u32);
        //         self.exchange_piece();
        //         return;
        //     }
        // }

        // 第三种，直接将每个碎片交换到其正确的位置
        // for i in 0..pos_len {
        //     let j = (i + pos_len / 3) % pos_len;
        //     if self.game_state.pos[j as usize] != j {
        //         self.game_state.exchange.push(j);

        //         let j_pos = self.game_state.pos.iter().position(|&r| r == j).unwrap() as u32;

        //         self.game_state.exchange.push(j_pos);
        //         self.exchange_piece();
        //         return;
        //     }
        // }

        // 第四种，螺旋矩阵
        for i in 0..pos_len {
            let j = self.game_state.recovery[i as usize];
            if self.game_state.pos[j as usize] != j {
                self.game_state.exchange.push(j);

                let j_pos = self.game_state.pos.iter().position(|&r| r == j).unwrap() as u32;

                self.game_state.exchange.push(j_pos);
                self.exchange_piece();
                return;
            }
        }
    }

    // 计算返回游戏进行时长
    pub fn get_elasp_time_str(&mut self) -> String {
        if !self.game_state.win {
            self.game_state.duration = self.game_state.start.elapsed();
        }

        if self.game_state.duration < TimeDelta::seconds(60) {
            format!("00:{:02}", self.game_state.duration.num_seconds())
        } else {
            let minutes = self.game_state.duration.num_minutes();
            let seconds = self.game_state.duration.num_seconds();
            format!("{:02}:{:02}", minutes, seconds)
        }
    }

    // 计算返回游戏剩余时间
    pub fn get_rest_time_str(&mut self) -> String {
        if !self.game_state.win && !self.game_state.end && self.game_state.challenge {
            self.game_state.rest = self.game_state.limit - self.game_state.start.elapsed();
        }

        if self.game_state.rest < TimeDelta::milliseconds(0) {
            self.game_state.rest = TimeDelta::milliseconds(0);
        }

        if self.game_state.challenge && self.game_state.rest == TimeDelta::milliseconds(0) {
            // 挑战模式时间耗尽,则结束
            self.game_state.end = true;
            "00:00".to_string()
        } else if self.game_state.rest < TimeDelta::seconds(60) {
            format!("{:.2}", self.game_state.rest.num_seconds())
        } else {
            let minutes = self.game_state.rest.num_minutes();
            let seconds = self.game_state.rest.num_seconds();
            format!("{:02}:{:02}", minutes, seconds)
        }
    }

    // 计算其他情况下的挑战模式限制
    pub fn calculate_time_limit(&self, x: f64) -> f64 {
        18.5 * x * x - 110.5 * x + 180.0
    }

    // 用于检查是否完成拼图，在每次交换碎片后被调用
    pub fn check_game(&mut self) {
        let mut flag = true;
        for i in 0..self.game_state.count * self.game_state.count {
            if self.game_state.pos[i as usize] != i {
                flag = false;
                break;
            }
        }
        if flag {
            self.game_state.win = true;
            self.game_state.end = true;
            playground::set_show_congrulation(true);
            #[cfg(feature = "debug")]
            println!("You win!");
        }
    }

    pub fn new(_cc: &eframe::CreationContext<'_>) -> anyhow::Result<Self> {
        //config::custom_font(cc);
        let app = std::panic::catch_unwind(|| GameApp {
            game_state: state::GameState::new(),
            ui_state: state::UiState {
                nav: state::Nav::Home,
            },
            img: imgs::ImageChoice::Image6,
        })
        .map_err(|_| anyhow::anyhow!("Failed to initialize game application"))?;
        Ok(app)
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.ui_state.nav {
            state::Nav::Home => {
                self.home(ctx, ui);
            }
            state::Nav::Game => {
                self.playground(ctx, ui);
                self.game_side(ctx, ui);
                if !self.game_state.end && self.game_state.bot {
                    self.recover();
                }
            }
        });
    }
}
