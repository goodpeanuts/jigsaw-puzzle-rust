/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-04 22:03:39
 * @FilePath: \puzzle\src\game.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use eframe::{egui::{self, Image, ColorImage, Color32}};
use egui_extras;
use eframe::CreationContext;
use eframe::egui::load::Bytes;
use image::{DynamicImage, GenericImageView, EncodableLayout, io::Reader};
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
        let mut img = image::load_from_memory(self.imgs.pic).expect("Failed to load image");
        let (width, height) = img.dimensions();
        let sub_width = width / self.count as u32;
        let sub_height = height / self.count as u32;
        for i in 0..self.count {
            for j in 0..self.count {
                let x = sub_width * i as u32;
                let y = sub_height * j as u32;
                print!("{}\n", i * self.count + j);
                let sub_image = img.crop(x, y, sub_width, sub_height);

                let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
                sub_image.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();

                let bytes = buf.into_inner();

                cc.include_bytes(format!("bytes://{}", i * self.count + j), GameApp::get_static_u8(&bytes));
                // let bytes_uri = format!("bytes://{}", i * self.count + j);
                cc.include_bytes(format!("bytes://{}", 99), self.imgs.pic);
            }
        }
    }


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
            egui_extras::install_image_loaders(ctx);
            //ctx.add_bytes_loader(loader::load_image);

            self.split_image(ctx);
            // self.process_pieces(ctx);
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