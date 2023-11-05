/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-05 22:06:39
 * @FilePath: \puzzle\src\state.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */

 pub enum Nav {
    Home,
    Game,
 }

 pub struct Piece{
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


#[derive(Default)]
pub struct GameState {
    pub init: bool,
    pub count: u32,
    pub pieces: Vec<Piece>,
    pub pos: Vec<u32>,
    pub exchange: Vec<u32>,
    pub win: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut gamestate = GameState {
            init: true,
            win: false,
            pieces: Vec::new(),
            pos: Vec::new(),
            exchange: Vec::new(),
            count: 3,
        };

        for i in 0..81 {
            gamestate.pieces.push(Piece::new(i));
        }
        gamestate
    }
}

pub struct UiState {
    pub nav: Nav,
}

