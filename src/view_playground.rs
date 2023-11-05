/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-05 22:15:38
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-06 00:45:20
 * @FilePath: \puzzle\src\view_playground.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use crate::game::GameApp;
use eframe::{egui::{self, Button}, epaint::vec2};
use std::io;
use std::io::Write;
use crate::state;

impl GameApp {
    pub fn playground(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if self.game_state.init {
           // 安装图片加载器,打乱拼图碎片
           //egui_extras::install_image_loaders(ctx);
           self.split_image(ctx);
           self.init_game();
       }
       
       // 计算大框矩形的位置
       let img_size = vec2(840.0 / self.game_state.count as f32, 840.0 / self.game_state.count as f32);
       let gap = 3.0; // 定义间隙宽度
       let offset = egui::pos2(30.0, 30.0); // 定义起点偏移量
       let total_gap = gap * (self.game_state.count as f32 - 1.0); // 总的间隙宽度
       let total_size = img_size.x * self.game_state.count as f32 + total_gap; // 总的大小
       let big_rect = egui::Rect::from_min_max(
            egui::pos2(30.0 - gap, 30.0 - gap), // 减去边缘的偏移量
           egui::pos2(30.0 + total_size + gap, 30.0 + total_size + gap), // 加上边缘的偏移量
       );
       ui.painter().rect_stroke(big_rect, 0.0, egui::Stroke::new(6.0, egui::Color32::WHITE));
       // --------------- diaplay image
       for i in 0..self.game_state.count {
           ui.spacing_mut().item_spacing = egui::Vec2::new(1.0, 1.0);
           ui.horizontal(|ui| {
               for j in 0..self.game_state.count {

                   let pos = i * self.game_state.count + j; 
                   let index = self.game_state.pos[pos as usize] as usize;
                   
                   /********** 用于调试 ************/
                   if self.game_state.init {
                       print!("[{:>2} - {:>2}] ", pos, index);
                   }
                   
                   // 计算拼图碎片的位置
                   let rect = egui::Rect::from_min_max(
                       egui::pos2(j as f32 * (img_size.x + gap) + offset.x, i as f32 * (img_size.y + gap) + offset.y),
                       egui::pos2((j + 1) as f32 * (img_size.x + gap) + offset.x, (i + 1) as f32 * (img_size.y + gap) + offset.y),
                   );
                   
                   ui.allocate_ui_at_rect(rect, |ui| {
                       ui.allocate_ui_at_rect(rect, |ui| {
                           let response = ui.add_sized([840.0 / self.game_state.count as f32, 840.0 / self.game_state.count as f32], egui::Image::from_uri(self.game_state.pieces[index as usize].uri.clone())).interact(egui::Sense::click());
                           
                           if response.hovered(){
                               ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                               
                           };

                           if response.clicked() {
                               /********** 用于调试 ************/
                               // 打印点击的位置
                               print!("{} ", pos);
                               io::stdout().flush().unwrap();

                               self.game_state.exchange.push(pos);
                               self.exchange_piece();
                           }

                           if self.game_state.exchange.contains(&pos) {
                               // ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(255, 0, 0));
                               let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
                               ui.painter().rect_stroke(response.rect, 0.0, stroke);
                           }
                           
                       });
                           
                   });
               }
               if self.game_state.init {
                   println!();
               }
           });
       }

       if self.game_state.win {
           self.congratulation(ctx, ui);
           //self.regretfulness(ctx, ui);
       }
       /********** 用于调试 ************/
       self.game_state.init = false;
   }
    fn congratulation(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Window::new("💕Congratulations").default_open(true).collapsible(false).fixed_pos([860.0, 380.0]).show(ctx, |ui| {
            ui.label(egui::RichText::new(
                "YOU MADE IT！")
            .size(20.0));

            let start_resp = ui.add_sized([100.0, 30.0], Button::new("Again")).on_hover_text("Back to menu").clicked();

            if start_resp {
                self.ui_state.nav = state::Nav::Home;
            }
            
        });
    }

    fn regretfulness(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Window::new("💔What a pity").default_open(true).collapsible(false).fixed_pos([860.0, 380.0]).show(ctx, |ui| {
            ui.label(egui::RichText::new(
                "You can be better next time")
            .size(20.0));

            let start_resp = ui.add_sized([100.0, 30.0], Button::new("Again")).on_hover_text("Back to menu").clicked();

            if start_resp {
                self.ui_state.nav = state::Nav::Home;
            }
            
        });
    }
}