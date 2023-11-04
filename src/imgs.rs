/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 21:18:49
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-04 23:37:22
 * @FilePath: \puzzle\src\imgs.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
// use eframe::egui;
// use image;
// use image::{DynamicImage, GenericImageView};

const IMAGE_1: &'static [u8] = include_bytes!("../assets/img/1.png");
const IMAGE_2: &'static [u8] = include_bytes!("../assets/img/2.png");
const IMAGE_3: &'static [u8] = include_bytes!("../assets/img/3.png");
const IMAGE_4: &'static [u8] = include_bytes!("../assets/img/4.png");
const IMAGE_5: &'static [u8] = include_bytes!("../assets/img/5.png");
const IMAGE_6: &'static [u8] = include_bytes!("../assets/img/6.png");
const IMAGE_7: &'static [u8] = include_bytes!("../assets/img/7.png");
const IMAGE_8: &'static [u8] = include_bytes!("../assets/img/8.jpg");
const IMAGE_9: &'static [u8] = include_bytes!("../assets/img/9.png");
const IMAGE_10: &'static [u8] = include_bytes!("../assets/img/10.png");
const IMAGE_11: &'static [u8] = include_bytes!("../assets/img/12.png");
const IMAGE_12: &'static [u8] = include_bytes!("../assets/img/12.png");


pub struct Images{
    pub pic1: &'static [u8],
    pub pic2: &'static [u8],
    pub pic3: &'static [u8],
    pub pic4: &'static [u8],
    pub pic5: &'static [u8],
    pub pic6: &'static [u8],
    pub pic7: &'static [u8],
    pub pic8: &'static [u8],
    pub pic9: &'static [u8],
    pub pic10: &'static [u8],
    pub pic11: &'static [u8],
    pub pic12: &'static [u8],   
}

impl Images {
    pub fn new() -> Self {
        Images {
            pic1: IMAGE_1,
            pic2: IMAGE_2,
            pic3: IMAGE_3,
            pic4: IMAGE_4,
            pic5: IMAGE_5,
            pic6: IMAGE_6,
            pic7: IMAGE_7,
            pic8: IMAGE_8,
            pic9: IMAGE_9,
            pic10: IMAGE_10,
            pic11: IMAGE_11,
            pic12: IMAGE_12 
        }
    }
    


}

// let img  = egui::Image::from_bytes("bytes://1", piece.as_bytes());
// ui.add(egui::Image::from_bytes("bytes://1", piece.as_bytes()));