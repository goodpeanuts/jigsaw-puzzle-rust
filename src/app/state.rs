/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2025-07-18 17:57:28
 * @FilePath: /jigsaw-puzzle-rust/src/app/state.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */

use crate::common::time::{TimeDelta, TimeStamp};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct GameState {
    pub init: bool,
    pub win: bool,
    pub end: bool,
    pub count: u32,
    pub pieces: Vec<Piece>,
    pub pos: Vec<u32>,
    pub exchange: Vec<u32>,
    pub challenge: bool,
    pub start: TimeStamp,
    pub duration: TimeDelta,
    pub limit: TimeDelta,
    pub rest: TimeDelta,
    pub is_custom: bool,
    pub custom_str: String,
    pub bot: bool,
    pub last_step_timestamp: TimeStamp, // 用于计算每一步走的时间
    pub recovery: Vec<u32>,

    // !
    pub index_offset: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            init: true,
            win: false,
            end: false,
            pieces: Vec::new(),
            pos: Vec::new(),
            exchange: Vec::new(),
            count: 3,
            challenge: false,
            start: TimeStamp::instant(),
            duration: TimeDelta::milliseconds(0.0),
            limit: TimeDelta::seconds(300.0),
            rest: TimeDelta::seconds(300.0),
            is_custom: false,
            custom_str: String::new(),
            bot: false,
            last_step_timestamp: TimeStamp::instant(),
            recovery: Vec::new(),

            // !
            index_offset: 0,
        }
    }

    pub fn create_pieces_index(&mut self) {
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
        self.start = TimeStamp::instant();
        self.duration = TimeDelta::milliseconds(0.0);
        self.limit = TimeDelta::seconds(300.0);
        self.rest = TimeDelta::seconds(300.0);
        self.is_custom = false;
        self.custom_str = String::new();
        self.bot = false;
        self.last_step_timestamp = TimeStamp::instant();
        self.recovery.clear();
    }
}

#[derive(Clone, Copy)]
pub struct UiState {
    pub nav: crate::views::Nav,
}
