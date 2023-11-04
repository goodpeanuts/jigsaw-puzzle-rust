/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 21:18:49
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-04 18:54:45
 * @FilePath: \puzzle\src\imgs.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
// use eframe::egui;
// use image;
// use image::{DynamicImage, GenericImageView};

const IMAGE_1: &'static [u8] = include_bytes!("../assets/img/6.png");

pub struct Images{
    pub img: String,
    pub pic: &'static [u8],
    
}

impl Images {
    pub fn new() -> Self {
        Images {
            img: "".to_string(),
            pic: IMAGE_1, 
        }
    }
    


}

// let img  = egui::Image::from_bytes("bytes://1", piece.as_bytes());
// ui.add(egui::Image::from_bytes("bytes://1", piece.as_bytes()));