/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-04 23:43:27
 * @FilePath: \puzzle\src\game.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use eframe::egui;
use egui_extras;
use image::GenericImageView;
use std::io::Cursor;

use crate::{config, state,imgs};



pub struct GameApp {
    pub state: state::GameState,
    imgs: imgs::Images,
    pub count: u32,
    pub pieces: Vec<Vec<u8>>,
    pub start: bool,
}

impl GameApp {

    pub fn split_image(&mut self, cc: &egui::Context) {
        let mut img = image::load_from_memory(self.imgs.pic6).expect("Failed to load image");
        let (width, height) = img.dimensions();
        let sub_width = width / self.count as u32;
        let sub_height = height / self.count as u32;
        for i in 0..self.count {
            for j in 0..self.count {
                let x = sub_width * i as u32;
                let y = sub_height * j as u32;
                print!("{}\n", i * self.count + j);
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

    // 将Vec<u8>转换为&'static [u8], 确保其拼图碎片拥有和程序一样长的生命周期
    fn get_static_u8(bytes: &Vec<u8>) -> &'static [u8] {
        let x = bytes.clone().into_boxed_slice();
        let static_ref = Box::leak(x);
        &static_ref[..]
    }

    
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        config::custom_font(cc);
        GameApp {
            state: state::GameState::default(),
            count: 9,
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
            self.start = true;
        }
        
        egui::CentralPanel::default().show(ctx, 
            |ui|{

                ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 1.0);

                for i in 0..self.count {
                    ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 1.0);
                    ui.horizontal(|ui| {
                        for j in 0..self.count {
                            ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 0.0);
                            let index = j * self.count + i; // Calculate the index for pieces vector

                                ui.add_sized([840.0 / self.count as f32, 840.0 / self.count as f32], egui::Image::from_uri(format!("bytes://{}", index)));
                        }
                    });
                }
            }); 
        
    }
}