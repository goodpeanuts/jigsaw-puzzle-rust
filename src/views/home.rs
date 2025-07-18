/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-05 22:23:38
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-18 14:22:09
 * @FilePath: /jigsaw-puzzle-rust/src/views/home.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::app::GameApp;
use crate::views::custom_widget::toggle;
use crate::{app::state, common::images};
use eframe::egui::{self, Button, Rect, UiBuilder};

impl GameApp {
    pub fn home(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.content(ctx, ui);
    }

    fn content(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
       //ui.add_sized([800.0, 900.0], egui::Image::from_uri(imgs::IMAGE_URI_background));
        // 这里分配一块空白区域，用于将文字和开始挤到中间
        ui.allocate_at_least(egui::vec2(1200.0, 400.0), egui::Sense::hover());
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                // ui.add_sized([25.0, 10.0], egui::widgets::Button::new("🔧"));
                ui.add_space(20.0);
                ui.vertical_centered_justified(|ui| {
                    // ui.label(egui::RichText::new("Jigsaw Puzzle").size(32.0).color(egui::Color32::LIGHT_GRAY));

                    if self.game_state().count < 5 {
                        ui.label(egui::RichText::new("✨ Jigsaw Puzzle").size(32.0));
                    } else if self.game_state().count < 8 {
                        ui.label(egui::RichText::new("🔥 Jigsaw Puzzle").size(32.0));
                    } else if self.game_state().count < 12 {
                        ui.label(egui::RichText::new("💀 Jigsaw Puzzle").size(32.0));
                    } else {
                        ui.label(egui::RichText::new("🎉 Jigsaw Puzzle").size(32.0));
                    }

                });

                ui.scope_builder(
                    UiBuilder::new().max_rect(egui::Rect::from_min_max(
                        egui::pos2(935.0, 570.0),
                        egui::pos2(1040.0, 820.0),
                    )),
                    |ui| {
                        ui.horizontal(|ui| {
                            //ui.add_space(450.0);
                            ui.add_sized([60.0, 60.0], egui::Label::new("Challenge"));
                            ui.add(toggle(&mut self.game_state().challenge))
                        });

                        if self.game_state().challenge {
                            ui.visuals_mut().widgets.hovered.weak_bg_fill =
                                egui::Color32::from_rgb(178, 102, 255);
                        } else {
                            ui.visuals_mut().widgets.hovered.weak_bg_fill =
                                egui::Color32::from_rgb(0, 204, 0);
                        }
                        let start_resp = ui
                            .add_sized(
                                [130.0, 45.0],
                                Button::new(egui::RichText::new("Start").size(16.0)),
                            )
                            .on_hover_text("can't wait to start")
                            .clicked();

                        if start_resp {
                            self.game_init(ctx, ui);
                            self.ui_state().nav = state::Nav::Game;
                        }

                        if self.game_state().challenge {
                            ui.label(
                                egui::RichText::new("Time is not unlimited")
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(178, 102, 255)),
                            );
                        }
                        //ui.spacing_mut().item_spacing = egui::Vec2::new(20.0, 20.0);

                        // 挑战模式开关
                        //ui.add_space(60.0);
                    },
                );

                let screen_rect = ui.ctx().available_rect(); // 当前窗口的可用区域
                let width = 105.0; // 控件宽度
                let height = 240.0; // 控件高度

                let min_x = screen_rect.right() - width; // 靠右对齐
                let min_y = 360.0; // 保持原来的 Y 位置
                let rect = egui::Rect::from_min_max(
                    egui::pos2(min_x, min_y),
                    egui::pos2(min_x + width, min_y + height),
                );
                ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add_space(14.0);

                        //ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(51,0,105);
                        ui.visuals_mut().widgets.hovered.weak_bg_fill =
                            egui::Color32::from_rgb(96, 96, 96);

                        if ui
                            .add_sized(
                                [80.0, 19.0],
                                Button::selectable(
                                    self.game_state().count == 3 && !self.game_state().is_custom,
                                    egui::RichText::new("easy").size(15.0),
                                ),
                            )
                            .clicked()
                        {
                            self.game_state().count = 3;
                            self.game_state().is_custom = false;
                        }

                        ui.add_space(9.0);

                        if ui
                            .add_sized(
                                [80.0, 19.0],
                                Button::selectable(
                                    self.game_state().count == 5 && !self.game_state().is_custom,
                                    egui::RichText::new("normal").size(15.0),
                                ),
                            )
                            .clicked()
                        {
                            self.game_state().count = 5;
                            self.game_state().is_custom = false;
                        }

                        ui.add_space(9.0);

                        if ui
                            .add_sized(
                                [80.0, 19.0],
                                Button::selectable(
                                    self.game_state().count == 8 && !self.game_state().is_custom,
                                    egui::RichText::new("difficult").size(15.0),
                                ),
                            )
                            .clicked()
                        {
                            self.game_state().count = 8;
                            self.game_state().is_custom = false;
                        }

                        ui.add_space(9.0);

                        if ui
                            .add_sized(
                                [80.0, 19.0],
                                Button::selectable(
                                    self.game_state().is_custom,
                                    egui::RichText::new("custom").size(15.0),
                                ),
                            )
                            .clicked()
                        {
                            self.game_state().is_custom = true;
                        }

                        // 自定义碎片数量
                        if self.game_state().is_custom {
                            ui.add_sized(
                                [100.0, 60.0],
                                egui::widgets::Slider::new(&mut self.game_state().count, 2..=12)
                                    .clamping(egui::SliderClamping::Never)
                                    .text(""),
                            );
                            if self.game_state().is_custom
                                && self.game_state().count >= 13
                                && self.game_state().count < 51
                            {
                                ui.label(
                                    egui::RichText::new("For Fun")
                                        .size(14.0)
                                        .color(egui::Color32::LIGHT_GREEN),
                                );
                            } else if self.game_state().is_custom && self.game_state().count >= 51 {
                                ui.label(
                                    egui::RichText::new("Not Recommend")
                                        .size(14.0)
                                        .color(egui::Color32::RED),
                                );
                            }
                        }
                    });
                });

                let rect_stroke = 7.0;
                let big_rect = egui::Rect::from_min_max(
                    egui::pos2(900.0, 40.0),   // 减去边缘的偏移量
                    egui::pos2(1140.0, 280.0), // 加上边缘的偏移量
                );

                if self.game_state().challenge {
                    ui.painter().rect_stroke(
                        big_rect,
                        0.0,
                        egui::Stroke::new(rect_stroke, egui::Color32::from_rgb(178, 102, 255)),
                        egui::StrokeKind::Middle,
                    );
                } else {
                    ui.painter().rect_stroke(
                        big_rect,
                        0.0,
                        egui::Stroke::new(rect_stroke, egui::Color32::LIGHT_GRAY),
                        egui::StrokeKind::Middle,
                    );
                }

                ui.scope_builder(
                    UiBuilder::new().max_rect(Rect::from_min_max(
                        egui::pos2(900.0, 40.0),
                        egui::pos2(1140.0, 280.0),
                    )),
                    |ui| {
                        ui.add_sized(
                            [240.0, 240.0],
                            egui::Image::from_uri(self.img().get_byte_uri()),
                        );
                    },
                );

                ui.scope_builder(
                    UiBuilder::new().max_rect(Rect::from_min_max(
                        egui::pos2(930.0, 280.0),
                        egui::pos2(990.0, 350.0),
                    )),
                    |ui| {
                        ui.horizontal_centered(|ui| {
                            egui::ComboBox::from_label("Select an image")
                                .selected_text(format!("{:?}", self.img().show_name()))
                                .width(120.0)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image1,
                                        "First",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_1),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image2,
                                        "Second",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_2),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image3,
                                        "Third",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_3),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image4,
                                        "Fourth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_4),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image5,
                                        "Fifth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_5),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image6,
                                        "Sixth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_6),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image7,
                                        "Seventh",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_7),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image8,
                                        "Eighth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_8),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image9,
                                        "Ninth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_9),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image10,
                                        "Tenth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_10),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image11,
                                        "Eleventh",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_11),
                                        );
                                    });
                                    ui.selectable_value(
                                        &mut *self.mut_img(),
                                        images::ImageChoice::Image12,
                                        "Twelfth",
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.add_sized(
                                            [200.0, 200.0],
                                            egui::Image::from_uri(images::IMAGE_URI_12),
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
