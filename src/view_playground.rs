/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-05 22:15:38
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-07 08:20:51
 * @FilePath: \puzzle\src\view_playground.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use crate::game::GameApp;
use eframe::{egui::{self, Button}, epaint::vec2};
use std::io;
use std::sync::Mutex;
use std::io::Write;
use crate::state;

// 这里的 SHOW_ORIGIN_IMAGE 如果设置在 GameApp 中，调用self.show_origin_image函数时会造成对变量的多次可变引用
// 如果定义直接定义在 self.game_side 会导致每次调用 self.game_side 都会重新初始化 SHOW_ORIGIN_IMAGE
// 如果想要设置一个可变全局静态变量，可以使用lazy_static宏，并且需要加锁访问
// 此外，设置其值时需要解锁，因此创建一个函数来设置其值
lazy_static::lazy_static! {
    static ref SHOW_ORIGIN_IMAGE: Mutex<bool> = Mutex::new(false);
}

fn set_show_origin_image(value: bool) {
    let mut show_origin_image = SHOW_ORIGIN_IMAGE.lock().unwrap();
    *show_origin_image = value;
}

impl GameApp {
    // 对局开始初始化，设置碎片个数并打乱，设置计时模式
    pub fn game_init(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // 如果是挑战模式，设置时间限制
        if self.game_state.challenge {
            match self.game_state.count {
                2 => {
                    self.game_state.limit = 10.0;
                }
                3 => {
                    self.game_state.limit = 15.0;
                },
                5 => {
                    self.game_state.limit = 90.0;
                },
                8 => {
                    self.game_state.limit = 480.0;
                },
                _ => {
                    self.game_state.limit = self.calculate_time_limit(self.game_state.count as f64);
                },        
            }
        }
        /********** 用于调试 ************/
        print!("1 set game_state.limit success\n");
        self.game_state.start = std::time::Instant::now();
        /********** 用于调试 ************/
        print!("2 set game_state.start success\n");
        self.shuffle_pieces();
        /********** 用于调试 ************/
        print!("3 shuffle_pieces success\n");
        self.split_image(ctx);
        /********** 用于调试 ************/
        print!("4 split_image success\n");

        // 防止有人设置碎片为 1*1 , (*/ω＼*)
        self.check_game();

        /********** 用于调试 ************/
        print!("5 init success\n");

        /********** 用于调试 ************/
        print!("init flase\n");
       self.game_state.init = false;
    }

    pub fn playground(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        
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

                   let pos = i * self.game_state.count + j;     // 界面中碎片的位置

                   // 将init设置为false的时机放在 game_init 的最后一步，否则这里就会因为重开时没有初始化访问到空数组
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
                           
                           if !self.game_state.bot && response.hovered(){
                               ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                               
                           };

                           if !self.game_state.bot && response.clicked() {
                               /********** 用于调试 ************/
                               // 打印点击的位置
                               print!("{} ", pos);
                               io::stdout().flush().unwrap();
                               self.game_state.exchange.push(pos);
                               ui.ctx().request_repaint();
                               self.exchange_piece();
                               ui.ctx().request_repaint();
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
        }   
        if self.game_state.challenge && self.game_state.end && !self.game_state.win {
            self.regretfulness(ctx, ui);
        }
   }

    pub fn game_side (&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // 刚开始时不显示原图
        if self.game_state.init {
            set_show_origin_image(false);
        }

        let game_side_rect = egui::Rect::from_min_max(
        egui::pos2(900.0, 20.0), 
        egui::pos2(1200.0, 900.0),
       );
        ui.allocate_ui_at_rect(
            game_side_rect, 
            |ui|{
                ui.allocate_ui_with_layout(ui.available_size(), 
                egui::Layout::top_down_justified(egui::Align::Center), 
            |ui|{

                /********** 用于调试 ************/
                // 画一个黄色填充的正方形
                // ui.painter().rect_filled(game_side_rect, 0.0, egui::Color32::from_rgb(255, 255, 0));
                // ui.spacing_mut().item_spacing = egui::Vec2::new(20.0, 20.0);
                ui.add_space(50.0);
                match self.game_state.count  {
                    3 => {
                        ui.label(egui::RichText::new(
                            "🔥Easy")
                        .size(25.0));
                    },
                    5 => {
                        ui.label(egui::RichText::new(
                            "🔥Normal")
                        .size(25.0));
                    },
                    8 => {
                        ui.label(egui::RichText::new(
                            "🔥Difficult")
                        .size(25.0));
                    },
                    _ => {
                        ui.label(egui::RichText::new(
                            "🔥Custom")
                        .size(25.0));
                    },
                }
                ui.add_space(50.0);

                // 这里重开一个ui，不然按钮的长度会因为justified被强制拉长至和layout一样长
                ui.vertical_centered(|ui|{
                    let show_imgea_resp = ui.add_sized([120.0, 40.0], 
                        egui::Button::new("查看原图")).on_hover_ui(|ui|{
                            ui.add_sized([200.0, 200.0], egui::Image::from_uri(self.img.get_byte_uri()));
                        });
        
                        if show_imgea_resp.clicked() {
                            set_show_origin_image(true);
                        }
        
                        if *SHOW_ORIGIN_IMAGE.lock().unwrap() {
                            self.show_origin_image(ctx, ui,  &mut *SHOW_ORIGIN_IMAGE.lock().unwrap());
                        }


                        let mut time_dispaly = String::new();
                        match self.game_state.challenge {
                            true => time_dispaly = self.get_rest_time_str(),
                            false => time_dispaly = self.get_elasp_time_str(),
                        };

                        // 根据时间长度调整字体大小
                        let size_font = |l: f32| -> f32 {
                            if l <= 22.0 {
                                25.0
                            } else {
                                let a = 330.0 / (l / 0.6);
                                if a > 12.0 {
                                    a
                                } else {
                                    12.0
                                }
                            }
                        };

                        // 根据剩余时间设置时间颜色
                        let mut time_color = egui::Color32::LIGHT_BLUE;

                        // 挑战模式时时间为红色和绿色
                        if self.game_state.challenge && self.game_state.rest < 21.0 {
                            time_color = egui::Color32::RED;
                        } else if self.game_state.challenge{
                            time_color = egui::Color32::LIGHT_GREEN;
                        } else {
                            // 非挑战模式时时间为蓝色
                            time_color = egui::Color32::LIGHT_BLUE;
                        }

                        ui.add_space(30.0);
                        ui.label(egui::RichText::new(time_dispaly)
                        .size(26.0)
                        .font(egui::FontId::monospace(size_font(5.0)))
                        .color(time_color));
                        // 请求重绘保证时间连续变化
                        ui.ctx().request_repaint();

                    let return_resp = ui.add_sized([120.0, 40.0], 
                            egui::Button::new("返回菜单"));
                            
                    if return_resp.clicked() {
                        self.ui_state.nav = state::Nav::Home;
                        self.game_state.reset_game_state();
                    }

                    let bot_resp = ui
                    .add_sized(
                        [80.0, 19.0],
                        egui::SelectableLabel::new(
                            self.game_state.bot,
                            egui::RichText::new("bot").size(15.0),
                        ),
                    );

                    if bot_resp.clicked() {
                        match self.game_state.bot {
                            true => self.game_state.bot = false,
                            false => self.game_state.bot = true,
                        }
                    }
                });
                    // ui.add_space(60.0);
            })
        });
    }

    fn show_origin_image(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, is_open: &mut bool) {
        egui::Window::new("Original image").title_bar(true).open(is_open).default_open(true).constrain(true).collapsible(false).movable(true).show(ctx, |ui| {
            ui.add(egui::Image::from_uri(self.img.get_byte_uri()));
        });
    }

    fn congratulation(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Window::new("💕Congratulations").default_open(true).collapsible(false).default_size(egui::vec2(1100.0, 850.0)).show(ctx, |ui| {
            ui.vertical_centered_justified(|ui|{
                ui.label(egui::RichText::new(
                    "YOU MADE IT！")
                .size(20.0));
                ui.add_sized([1200.0, 900.0], egui::Image::from_uri(self.img.get_byte_uri()));
            });

            let restart_resp = ui.add_sized([100.0, 30.0], Button::new("Again")).on_hover_text("Back to menu").clicked();

            if restart_resp {
                self.ui_state.nav = state::Nav::Home;
                self.game_state.reset_game_state();
            }
            
        });
    }

    fn regretfulness(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Window::new("💔What a pity").default_open(true).collapsible(false).fixed_pos([860.0, 380.0]).show(ctx, |ui| {
            ui.label(egui::RichText::new(
                "You can be better next time")
            .size(20.0));

            let restart_resp = ui.add_sized([100.0, 30.0], Button::new("Again")).on_hover_text("Back to menu").clicked();

            if restart_resp {
                self.ui_state.nav = state::Nav::Home;
                self.game_state.reset_game_state();
            }
            
        });
    }
}