/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-05 22:23:38
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-21 16:28:25
 * @FilePath: /jigsaw-puzzle-rust/src/views/home.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crate::app::GameApp;
use crate::common::images;
use crate::views::custom_widget::toggle;
use eframe::egui::{self, Button, CentralPanel, UiBuilder, Vec2};
use egui_extras::{Size, StripBuilder};

impl GameApp {
    pub fn home(&mut self, ctx: &egui::Context, _ui: &mut egui::Ui) {
        CentralPanel::default().show(ctx, |ui| {
            let full = ui.available_size();
            let margin_x = full.x * 0.06;
            let margin_y = full.y * 0.06;
            let margin = f32::max(margin_x, margin_y);
            let full_rect = egui::Rect::from_min_max(ui.min_rect().min, ui.min_rect().max);

            ui.scope_builder(UiBuilder::new().max_rect(full_rect), |ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.25)) // 左列
                    .size(Size::relative(0.50)) // 中列
                    .size(Size::relative(0.25)) // 右列
                    .horizontal(|mut strip| {
                        strip.cell(|_| {}); // 左列空
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.label(egui::RichText::new("✨ Jigsaw Puzzle").size(32.0));
                            });
                        });
                        strip.cell(|ui| {
                            egui::Frame::new()
                                .fill(egui::Color32::from_additive_luminance(8))
                                .inner_margin(margin)
                                .outer_margin(margin * 0.05)
                                .corner_radius(10.0) // 圆角
                                .inner_margin(8.0) // 内边距
                                .show(ui, |ui| {
                                    ui.centered_and_justified(|ui| self.right(ctx, ui));
                                });
                        });
                    });
            })
        });
    }

    fn right(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        StripBuilder::new(ui)
            .size(Size::relative(0.6)) // 上：图像区域占 60%
            .size(Size::relative(0.2)) // 中：模式选择
            .size(Size::relative(0.2)) // 下：开始区
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    self.show_image(ui);
                    self.choose_image(ui);
                });
                strip.cell(|ui| {
                    self.game_mode_choice(ui);
                });
                strip.cell(|ui| {
                    self.game_start(ctx, ui);
                });
            });
    }

    fn show_image(&mut self, ui: &mut egui::Ui) {
        let rect_stroke = 7.0;

        // 获取当前列的宽度（右侧 strip cell）
        let available_width = ui.available_width();

        // 图片最大边长 = 当前列宽度，保留边框空间
        let side = available_width - rect_stroke * 2.0;

        // 定义图片大小（正方形）
        let image_size = egui::vec2(side, side);

        // 用垂直布局，上面是图片，下面可以继续放按钮或其他控件
        ui.vertical_centered_justified(|ui| {
            // 绘制边框 + 图片
            let (response, painter) = ui.allocate_painter(image_size, egui::Sense::hover());

            let image_rect = response.rect;

            // 绘制图片
            egui::Image::new(self.img().get_byte_uri())
                .corner_radius(5)
                .tint(egui::Color32::WHITE)
                .paint_at(ui, image_rect);

            // 绘制边框（仅围绕图片）
            let stroke_color = if self.game_state().challenge {
                egui::Color32::from_rgb(178, 102, 255)
            } else {
                egui::Color32::LIGHT_GRAY
            };
            painter.rect_stroke(
                image_rect,
                5.0,
                egui::Stroke::new(rect_stroke, stroke_color),
                egui::StrokeKind::Middle,
            );
        });
    }

    fn choose_image(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            egui::ComboBox::from_label("Select an image")
                .selected_text(format!("{:?}", self.img().show_name()))
                .width(120.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut *self.mut_img(), images::ImageChoice::Image1, "First")
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
                        ui.add_sized([200.0, 200.0], egui::Image::from_uri(images::IMAGE_URI_2));
                    });
                    ui.selectable_value(&mut *self.mut_img(), images::ImageChoice::Image3, "Third")
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
                        ui.add_sized([200.0, 200.0], egui::Image::from_uri(images::IMAGE_URI_4));
                    });
                    ui.selectable_value(&mut *self.mut_img(), images::ImageChoice::Image5, "Fifth")
                        .on_hover_ui(|ui| {
                            ui.add_sized(
                                [200.0, 200.0],
                                egui::Image::from_uri(images::IMAGE_URI_5),
                            );
                        });
                    ui.selectable_value(&mut *self.mut_img(), images::ImageChoice::Image6, "Sixth")
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
                        ui.add_sized([200.0, 200.0], egui::Image::from_uri(images::IMAGE_URI_7));
                    });
                    ui.selectable_value(
                        &mut *self.mut_img(),
                        images::ImageChoice::Image8,
                        "Eighth",
                    )
                    .on_hover_ui(|ui| {
                        ui.add_sized([200.0, 200.0], egui::Image::from_uri(images::IMAGE_URI_8));
                    });
                    ui.selectable_value(&mut *self.mut_img(), images::ImageChoice::Image9, "Ninth")
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
                        ui.add_sized([200.0, 200.0], egui::Image::from_uri(images::IMAGE_URI_10));
                    });
                    ui.selectable_value(
                        &mut *self.mut_img(),
                        images::ImageChoice::Image11,
                        "Eleventh",
                    )
                    .on_hover_ui(|ui| {
                        ui.add_sized([200.0, 200.0], egui::Image::from_uri(images::IMAGE_URI_11));
                    });
                    ui.selectable_value(
                        &mut *self.mut_img(),
                        images::ImageChoice::Image12,
                        "Twelfth",
                    )
                    .on_hover_ui(|ui| {
                        ui.add_sized([200.0, 200.0], egui::Image::from_uri(images::IMAGE_URI_12));
                    });
                });
        });
    }

    fn game_mode_choice(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(14.0);

            ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(96, 96, 96);

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
    }

    fn game_start(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let avail = ui.available_size();
        let col_width = ui.available_width(); // 当前列的宽度
        let col_height = avail.y;

        egui::Grid::new("challenge_grid")
            .num_columns(2)
            .spacing(Vec2::new(col_width * 0.3, col_height * 0.3))
            .striped(true)
            .show(ui, |ui| {
                ui.label("Challenge");

                ui.add(toggle(&mut self.game_state().challenge));
            });

        // 动态尺寸
        let button_width = col_width * 0.4;
        let button_height = col_height * 0.4;
        let spacing = col_height * 0.02;
        ui.vertical_centered(|ui| {
            let start_resp = ui
                .add_sized(
                    [button_width, button_height],
                    Button::new(egui::RichText::new("Start").size(button_width * 0.25)),
                )
                .clicked();

            if start_resp {
                self.game_init(ctx, ui);
                self.ui_state().nav = super::Nav::Game;
            }

            ui.add_space(spacing);

            if self.game_state().challenge {
                ui.label(
                    egui::RichText::new("Time is not unlimited")
                        .size(button_height * 0.2)
                        .color(egui::Color32::from_rgb(178, 102, 255)),
                );
            }
        });
    }
}
