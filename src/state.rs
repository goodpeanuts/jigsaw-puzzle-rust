/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-07 00:22:59
 * @FilePath: \puzzle\src\state.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use std::time;

pub enum Nav {
    Home,
    Game,
}

pub struct Piece {
    pub id: u32,
    pub uri: String,
}

impl Piece {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            uri: format!("bytes://{}", id),
        }
    }
}

pub struct GameState {
    pub init: bool,
    pub win: bool,
    pub end: bool,
    pub count: u32,
    pub pieces: Vec<Piece>,
    pub pos: Vec<u32>,
    pub exchange: Vec<u32>,
    pub challenge: bool,
    pub start: std::time::Instant,
    pub duration: f64,
    pub limit: f64,
    pub rest: f64,
    pub custom: bool,
    pub custom_str: String,
}

impl GameState {
    pub fn new() -> Self {
        let mut gamestate = GameState {
            init: true,
            win: false,
            end: false,
            pieces: Vec::new(),
            pos: Vec::new(),
            exchange: Vec::new(),
            count: 3,
            challenge: false,
            start: std::time::Instant::now(),
            duration: 0.0,
            limit: 300.0,
            rest: 300.0,
            custom: false,
            custom_str: String::new(),
        };
        gamestate
    }

    pub fn create_pieces_index (&mut self) {
        for i in 0..self.count * self.count {
            self.pieces.push(Piece::new(i));
        }
    }

    pub fn reset_game_state(&mut self) {
        self.init = true;
        self.win = false;
        self.end = false;
        self.pieces.clear();
        self.pos.clear();
        self.exchange.clear();
        self.count = 3;
        self.challenge = false;
        self.start = time::Instant::now();
        self.duration = 0.0;
        self.limit = 300.0;
        self.rest = 300.0;
        self.custom = false;
        self.custom_str = String::new();
    }
}

pub struct UiState {
    pub nav: Nav,
}
