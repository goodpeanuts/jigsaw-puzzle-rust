/*
 * @Author: goodpeanuts goodpeanuts@foxmail.com
 * @Date: 2023-11-03 14:35:18
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-11-05 15:54:23
 * @FilePath: \puzzle\src\state.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */


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
    pub start: bool,
    pub pieces: Vec<Piece>,
    pub pos: Vec<u32>,
    pub exchange: Vec<u32>,
}

impl GameState {
    pub fn new() -> Self {
        let mut gamestate = GameState {
            start: false,
            pieces: Vec::new(),
            pos: Vec::new(),
            exchange: Vec::new(),
        };

        for i in 0..81 {
            gamestate.pieces.push(Piece::new(i));
        }
        gamestate
    }
    
}

