/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-05 17:10:36
 * @FilePath: \puzzle\src\game.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use eframe::{egui::{self, Sense}, epaint::vec2};
use egui_extras;
use image::GenericImageView;
use std::{io::Cursor, process::id};
use rand::prelude::SliceRandom;
use std::io;
use std::io::Write;

use crate::{config, state,imgs};



pub struct GameApp {
    pub state: state::GameState,
    imgs: imgs::Images,
    pub count: u32,
    pub pieces: Vec<Vec<u8>>,
    pub start: bool,
}

impl GameApp {

    // 将Vec<u8>转换为&'static [u8], 确保其拼图碎片拥有和程序一样长的生命周期
    fn get_static_u8(bytes: &Vec<u8>) -> &'static [u8] {
        let x = bytes.clone().into_boxed_slice();
        let static_ref = Box::leak(x);
        &static_ref[..]
    }

    pub fn split_image(&mut self, cc: &egui::Context) {
        let mut img = image::load_from_memory(self.imgs.pic6).expect("Failed to load image");
        let (width, height) = img.dimensions();
        let sub_width = width / self.count as u32;
        let sub_height = height / self.count as u32;
        for i in 0..self.count {
            for j in 0..self.count {
                let x = sub_width * j as u32;
                let y = sub_height * i as u32;
                //print!("{}", i * self.count + j);
                // 从原图中裁剪出子图
                let sub_image = img.crop(x, y, sub_width, sub_height);

                // 将处理好的图片写入到缓存中
                let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
                sub_image.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
                let bytes = buf.into_inner();

                // 将缓存中的图片写入到egui的缓存中
                cc.include_bytes(format!("bytes://{}", i * self.count + j), GameApp::get_static_u8(&bytes));
            }
        }
    }

    // 初始化游戏，打乱碎片
    pub fn init_game(&mut self) {
        for i in 0..self.count * self.count {
            self.state.pos.push(i);
        }
        self.state.pos.shuffle(&mut rand::thread_rng());
    }

    pub fn exchange_piece(&mut self) {
        if self.state.exchange.len() == 2 {
            print!("[{:>2} - {:>2}]  exchanged\n", self.state.exchange[0], self.state.exchange[1]);
            self.state.pos.swap(self.state.exchange[0].try_into().unwrap(), self.state.exchange[1].try_into().unwrap());
            self.state.exchange.clear();
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        config::custom_font(cc);
        GameApp {
            state: state::GameState::new(),
            count: 5,
            imgs: imgs::Images::new(),
            pieces: vec![],
            start: false,
        }
    }

    pub fn setup(&mut self, cc: &egui::Context) {
        //egui_extras::install_image_loaders(cc);
    }
}   

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        
 
        if !self.start {
            // 安装图片加载器
            egui_extras::install_image_loaders(ctx);
            self.split_image(ctx);
            self.init_game();
        }
        
        egui::CentralPanel::default().show(ctx, 
            |ui|{

            //  --------------- try grid

            // --------------- diaplay image
                ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 1.0);

                for i in 0..self.count {
                    ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 1.0);
                    ui.horizontal(|ui| {
                        for j in 0..self.count {

                            let pos = i * self.count + j; 
                            let index = self.state.pos[pos as usize] as usize;
                            /********** 用于调试 ************/
                            if !self.start {

                                print!("[{:>2} - {:>2}] ", pos, index);
                            }
                            /********** 用于调试 ************/
                            
                            let img_size = vec2(840.0 / self.count as f32, 840.0 / self.count as f32);

                            // 计算拼图碎片的位置
                            let gap = 3.0; // 定义间隙宽度
                            let rect = egui::Rect::from_min_max(
                                egui::pos2(j as f32 * (img_size.x + gap), i as f32 * (img_size.y + gap)),
                                egui::pos2((j + 1) as f32 * (img_size.x + gap), (i + 1) as f32 * (img_size.y + gap)),
                            );
                            
                            ui.allocate_ui_at_rect(rect, |ui| {
                                // .on_hover_ui(|ui| {
                                //     // 鼠标变成手指
                                //     ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                                //     ui.image("bytes://0");
                                // })
     
                                // if ui.add_sized([840.0 / self.count as f32, 840.0 / self.count as f32], egui::Image::from_uri(self.state.pieces[index as usize].uri.clone())).interact(egui::Sense::click()).clicked(){
                                //     // 打印点击的位置
                                //     print!("{} ", pos);
                                //     ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                                //     io::stdout().flush().unwrap();
                                // };
                                ui.allocate_ui_at_rect(rect, |ui| {
                                    let response = ui.add_sized([840.0 / self.count as f32, 840.0 / self.count as f32], egui::Image::from_uri(self.state.pieces[index as usize].uri.clone())).interact(egui::Sense::click());
                                    
                                    if response.hovered(){
                                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                        
                                    };

                                    if response.clicked() {
                                        // 打印点击的位置
                                        print!("{} ", pos);
                                        io::stdout().flush().unwrap();

                                        self.state.exchange.push(pos);
                                        self.exchange_piece();
                                    }

                                    if self.state.exchange.contains(&pos) {
                                        // ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(255, 0, 0));
                                        let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
                                        ui.painter().rect_stroke(response.rect, 0.0, stroke);
                                    }
                                    
                                });
                                    
                            });
                        }

                        if !self.start {
                            println!();
                        }
                    });
                }
            }); 
            self.start = true;
    }
}