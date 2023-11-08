/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 21:18:49
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-08 20:35:25
 * @FilePath: \puzzle\src\imgs.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
// use eframe::egui;
// use image;
// use image::{DynamicImage, GenericImageView};

pub const IMAGE_background: &'static [u8] = include_bytes!("../assets/img/bg.jpg");
pub const IMAGE_1: &'static [u8] = include_bytes!("../assets/img/1.png");
pub const IMAGE_2: &'static [u8] = include_bytes!("../assets/img/2.png");
pub const IMAGE_3: &'static [u8] = include_bytes!("../assets/img/3.png");
pub const IMAGE_4: &'static [u8] = include_bytes!("../assets/img/4.png");
pub const IMAGE_5: &'static [u8] = include_bytes!("../assets/img/5.png");
pub const IMAGE_6: &'static [u8] = include_bytes!("../assets/img/6.png");
pub const IMAGE_7: &'static [u8] = include_bytes!("../assets/img/7.png");
pub const IMAGE_8: &'static [u8] = include_bytes!("../assets/img/8.jpg");
pub const IMAGE_9: &'static [u8] = include_bytes!("../assets/img/9.png");
pub const IMAGE_10: &'static [u8] = include_bytes!("../assets/img/10.png");
pub const IMAGE_11: &'static [u8] = include_bytes!("../assets/img/11.png");
pub const IMAGE_12: &'static [u8] = include_bytes!("../assets/img/12.png");

pub const IMAGE_URI_background: &'static str = "bytes://background";
pub const IMAGE_URI_1: &'static str = "bytes://x1";
pub const IMAGE_URI_2: &'static str = "bytes://x2";
pub const IMAGE_URI_3: &'static str = "bytes://x3";
pub const IMAGE_URI_4: &'static str = "bytes://x4";
pub const IMAGE_URI_5: &'static str = "bytes://x5";
pub const IMAGE_URI_6: &'static str = "bytes://x6";
pub const IMAGE_URI_7: &'static str = "bytes://x7";
pub const IMAGE_URI_8: &'static str = "bytes://x8";
pub const IMAGE_URI_9: &'static str = "bytes://x9";
pub const IMAGE_URI_10: &'static str = "bytes://x10";
pub const IMAGE_URI_11: &'static str = "bytes://x11";
pub const IMAGE_URI_12: &'static str = "bytes://x12";


#[derive(Clone, Copy, PartialEq)]
pub enum ImageChoice {
    Image1,
    Image2,
    Image3,
    Image4,
    Image5,
    Image6,
    Image7,
    Image8,
    Image9,
    Image10,
    Image11,
    Image12, 
}

impl ImageChoice {
    pub fn chose_pic(&self) -> &'static [u8] {
        match self {
            ImageChoice::Image1 => IMAGE_1,
            ImageChoice::Image2 => IMAGE_2,
            ImageChoice::Image3 => IMAGE_3,
            ImageChoice::Image4 => IMAGE_4,
            ImageChoice::Image5 => IMAGE_5,
            ImageChoice::Image6 => IMAGE_6,
            ImageChoice::Image7 => IMAGE_7,
            ImageChoice::Image8 => IMAGE_8,
            ImageChoice::Image9 => IMAGE_9,
            ImageChoice::Image10 => IMAGE_10,
            ImageChoice::Image11 => IMAGE_11,
            ImageChoice::Image12 => IMAGE_12,
        }
    }
    pub fn show_name(&self) -> String {
        match self {
            ImageChoice::Image1 => "1.png".to_string(),
            ImageChoice::Image2 => "2.png".to_string(),
            ImageChoice::Image3 => "3.png".to_string(),
            ImageChoice::Image4 => "4.png".to_string(),
            ImageChoice::Image5 => "5.png".to_string(),
            ImageChoice::Image6 => "6.png".to_string(),
            ImageChoice::Image7 => "7.png".to_string(),
            ImageChoice::Image8 => "8.jpg".to_string(),
            ImageChoice::Image9 => "9.png".to_string(),
            ImageChoice::Image10 => "10.png".to_string(),
            ImageChoice::Image11 => "11.png".to_string(),
            ImageChoice::Image12 => "12.png".to_string(),
        }
    }

    pub fn get_byte_uri(self) -> String {
        match self {
            ImageChoice::Image1 => IMAGE_URI_1.to_string(),
            ImageChoice::Image2 => IMAGE_URI_2.to_string(),
            ImageChoice::Image3 => IMAGE_URI_3.to_string(),
            ImageChoice::Image4 => IMAGE_URI_4.to_string(),
            ImageChoice::Image5 => IMAGE_URI_5.to_string(),
            ImageChoice::Image6 => IMAGE_URI_6.to_string(),
            ImageChoice::Image7 => IMAGE_URI_7.to_string(),
            ImageChoice::Image8 => IMAGE_URI_8.to_string(),
            ImageChoice::Image9 => IMAGE_URI_9.to_string(),
            ImageChoice::Image10 => IMAGE_URI_10.to_string(),
            ImageChoice::Image11 => IMAGE_URI_11.to_string(),
            ImageChoice::Image12 => IMAGE_URI_12.to_string(),
        }
    }
}