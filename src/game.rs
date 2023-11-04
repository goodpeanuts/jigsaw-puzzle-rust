use eframe::{egui::{self, Image}, epaint::vec2};
use egui_extras;
use eframe::CreationContext;
use eframe::egui::load::Bytes;
use image::{DynamicImage, GenericImageView, EncodableLayout};

use crate::{config, state,imgs};


pub struct GameApp<'a> {
    pub state: state::GameState,
    imgs: imgs::Images,
    pub count: u32,
    pub pieces: Vec<egui::Image<'a>>,
}

impl <'a>GameApp<'a> {
    pub fn split_image(&mut self, cc: &egui::Context) {
        let mut img = image::load_from_memory(self.imgs.pic).expect("Failed to load image");
        let (width, height) = img.dimensions();
        let sub_width = width / self.count as u32;
        let sub_height = height / self.count as u32;
        for i in 0..self.count {
            for j in 0..self.count {
                let x = sub_width * i as u32;
                let y = sub_height * j as u32;
                let sub_image = img.crop(x, y, sub_width, sub_height);
                // 将临时值存储到一个变量中，以延长它的生命周期
                let sub_image_bytes = sub_image.as_bytes();
                let sub_image_data: Vec<u8> = sub_image_bytes.to_vec(); // 将 &[u8] 转换成 Vec<u8>

                let sub_image1 = egui::Image::from_bytes((i + j).to_string(), sub_image_data.clone());

                cc.include_bytes((i+j).to_string(), sub_image_data.clone());
                self.pieces.push(sub_image1.clone());
            }
        }
    }

    pub fn to_uri(&self, index: usize) -> String {
        format!("bytes://{}", index)
    }
    
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        config::custom_font(cc);
        GameApp {
            state: state::GameState::default(),
            count: 3,
            imgs: imgs::Images::new(),
            pieces: vec![],
        }
    }

    pub fn setup(&mut self, cc: &egui::Context) {
        egui_extras::install_image_loaders(cc);
 
        self.split_image(cc);

        let mut img = image::load_from_memory(self.imgs.pic).expect("Failed to load image");
        let image_bytes = img.as_bytes();
        let image_data: Vec<u8> = image_bytes.to_vec(); // 将 &[u8] 转换成 Vec<u8>

        // let image1 = egui::Image::from_bytes("bytes://1", image_data.clone());

        cc.include_bytes("bytes://1", image_data.clone());

    }
}   

impl <'a>eframe::App for GameApp<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        egui::CentralPanel::default().show(ctx, 
            |ui|{
                ctx.include_bytes("bytes://1", self.imgs.pic.as_bytes().to_vec());
                ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 0.0);
                // for _i in 0..3 {
                //     ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 0.0);
                //     ui.horizontal(|ui|{
                //         for _j in 0..3 {
                //             ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 0.0);
                //             ui.add_sized([200.0, 200.0], egui::Image::new(egui::include_image!("../assets/img/0.jpg")).max_size(vec2(200.0, 200.0)));
                //         } 
                //     });
                //     //ui.end_row();
                // }

                for i in 0..3 {
                    ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 0.0);
                    ui.horizontal(|ui| {
                        for j in 0..3 {
                            ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 0.0);
                            let index = i * 3 + j; // Calculate the index for pieces vector

                                let idx = index.to_string();
                                ui.add_sized([280.0, 280.0], egui::Image::from_uri("bytes://1"));
                        }
                    });
                }
                

            }); 
        
    }
}