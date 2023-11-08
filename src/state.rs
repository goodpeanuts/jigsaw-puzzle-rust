/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-09 00:35:47
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
    pub fn new(id: u32, offset: u32) -> Self {
        Self {
            id,
            uri: format!("bytes://{}", id + offset),
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
    pub bot: bool,
    pub step_time: time::Instant, // 用于计算每一步走的时间
    pub recovery: Vec<u32>,

    // !
    pub index_offset: u32,
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
            bot: false,
            step_time: time::Instant::now(),
            recovery: Vec::new(),

            // !
            index_offset: 0,
        };
        gamestate
    }

    pub fn create_pieces_index (&mut self) {
        for i in 0..self.count * self.count {
            self.pieces.push(Piece::new(i, self.index_offset));
        }
    }

    pub fn reset_game_state(&mut self) {
        // !
        self.index_offset += self.count * self.count;
        
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
        self.bot = false;
        self.step_time = time::Instant::now();
        self.recovery.clear();
    }
}

pub struct UiState {
    pub nav: Nav,
}
