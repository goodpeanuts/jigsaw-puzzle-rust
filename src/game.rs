use eframe::egui;

use crate::config;

pub struct Game {
    //pub state: state::State,
    //pub playground: playground::Playground,
}

impl Game {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        config::custom_font(cc);
        Game {
            //state: state::State::new(),
            //playground: playground::Playground::new(),
        }
    }

    
}   