/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-05 22:23:38
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-07 09:04:45
 * @FilePath: \puzzle\src\view_home.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::game::GameApp;
use crate::{imgs, state};
use eframe::{
    egui::{self, Button},
    epaint::vec2,
};
use crate::custom_widget::{self, toggle, toggle_ui};

impl GameApp {
    pub fn home(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // 这里分配一块空白区域，用于将文字和开始挤到中间
        ui.allocate_at_least(egui::vec2(1200.0, 400.0), egui::Sense::hover());
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                // ui.add_sized([25.0, 10.0], egui::widgets::Button::new("🔧"));
                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("🔥Jigsaw Puzzle:").size(28.0));

                    ui.add_space(80.0);
                    let start_resp = ui
                        .add_sized([150.0, 60.0], Button::new("Start"))
                        .on_hover_text("Start the game")
                        .clicked();

                    if start_resp {
                        /********** 用于调试 ************/
                        self.game_init(ctx, ui);
                        self.ui_state.nav = state::Nav::Game;
                    }
                    ui.spacing_mut().item_spacing = egui::Vec2::new(20.0, 20.0);

                    ui.horizontal(|ui| {
                        ui.add_space(450.0);
                        if ui
                            .add_sized(
                                [80.0, 19.0],
                                egui::SelectableLabel::new(
                                    self.game_state.count == 3 && self.game_state.custom == false,
                                    egui::RichText::new("easy").size(15.0),
                                ),
                            )
                            .clicked()
                        {
                            self.game_state.count = 3;
                            self.game_state.custom = false;
                        }

                        if ui
                            .add_sized(
                                [80.0, 19.0],
                                egui::SelectableLabel::new(
                                    self.game_state.count == 5 && self.game_state.custom == false,
                                    egui::RichText::new("normal").size(15.0),
                                ),
                            )
                            .clicked()
                        {
                            self.game_state.count = 5;
                            self.game_state.custom = false;
                        }
                        if ui
                            .add_sized(
                                [80.0, 19.0],
                                egui::SelectableLabel::new(
                                    self.game_state.count == 8 && self.game_state.custom == false,
                                    egui::RichText::new("difficult").size(15.0),
                                ),
                            )
                            .clicked()
                        {
                            self.game_state.count = 8;
                            self.game_state.custom = false;
                        }
                        if ui
                            .add_sized(
                                [80.0, 19.0],
                                egui::SelectableLabel::new(
                                    self.game_state.custom == true,
                                    egui::RichText::new("custom").size(15.0),
                                ),
                            )
                            .clicked()
                        {
                            self.game_state.custom = true;
                            
                        }
                
                        
                        ui.set_visible(self.game_state.custom);
                        ui.add_sized([40.0, 40.0], egui::widgets::Slider::new(&mut self.game_state.count, 2..=12).clamp_to_range(true).text(""));
                        
                    });

                    // 挑战模式开关
                    ui.add_space(60.0);
                    ui.horizontal(|ui|{
                        ui.add_space(450.0);
                        ui.add_sized([60.0, 60.0], egui::Label::new("Challenge"));
                        ui.add(toggle(&mut self.game_state.challenge))
                    });
                    
                });

                ui.add_space(20.0);

                // 选择图片的多选框
                ui.allocate_ui_at_rect(
                    egui::Rect::from_min_max(
                        egui::pos2(520.0, 640.0), 
                        egui::pos2(690.0, 690.0), 
                    ),
                    |ui| {
                        ui.horizontal_centered(|ui| {
                            egui::ComboBox::from_label("Select an image")
                                .selected_text(format!("{:?}", self.img.show_name()))
                                .width(120.0)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image1,
                                        "First",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_1),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image2,
                                        "Second",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_2),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image3,
                                        "Third",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_3),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image4,
                                        "Fourth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_4),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image5,
                                        "Fifth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_5),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image6,
                                        "Sixth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_6),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image7,
                                        "Seventh",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_7),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image8,
                                        "Eighth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_8),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image9,
                                        "Ninth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_9),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image10,
                                        "Tenth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_10),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image11,
                                        "Eleventh",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_11),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut self.img,
                                        imgs::ImageChoice::Image12,
                                        "Twelfth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(imgs::IMAGE_URI_12),
                                        );
                                    });
                                });
                        });
                    },
                );
            },
        );
    }
}
