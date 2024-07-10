use macroquad::prelude::*;
use crate::constants::*;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Route {
    NS,
    SN,
    WE,
    EW,
    NW,
    SE,
    WS,
    EN,
    NE,
    SW,
    WN,
    ES,
}

impl Route {
    pub fn get_coordinates(&self) -> Vec2 {
        match *self {
            Route::NS => vec2(410.0, 0.0),
            Route::SN => vec2(550.0, 1000.0),
            Route::WE => vec2(0.0, 560.0),
            Route::EW => vec2(1000.0, 420.0),
            Route::NW => vec2(360.0, 0.0),
            Route::SE => vec2(600.0, 1000.0),
            Route::WS => vec2(0.0, 605.0),
            Route::EN => vec2(1000.0, 365.0),
            Route::NE => vec2(460.0, 0.0),
            Route::SW => vec2(500.0, 1000.0),
            Route::WN => vec2(0.0, 515.0),
            Route::ES => vec2(1000.0, 465.0),
        }
    }

    pub fn get_speed(&self) -> (f32, f32) {
        match *self {
            Route::NS | Route::NW | Route::NE => (0.0, VITESSE_NORMAL),
            Route::SN | Route::SE | Route::SW => (0.0, -VITESSE_NORMAL),
            Route::WE | Route::WS | Route::WN => (VITESSE_NORMAL, 0.0),
            Route::EW | Route::EN | Route::ES => (-VITESSE_NORMAL, 0.0),
        }
    }

    pub fn get_direction(&self) -> Direction {
        match *self {
            Route::NS | Route::NW | Route::NE => Direction::Down,
            Route::SN | Route::SE | Route::SW => Direction::Up,
            Route::WE | Route::WS | Route::WN => Direction::Right,
            Route::EW | Route::EN | Route::ES => Direction::Left,
        }
    }

    pub fn not_allowed_to_go(&self) -> Vec<Route> {
        match *self {
            Route::NS => vec![Route::EW, Route::WE, Route::WN, Route::SW],
            Route::SN => vec![Route::NE, Route::WE, Route::ES, Route::EW],
            Route::WE => vec![Route::NS, Route::SW, Route::SN, Route::ES],
            Route::EW => vec![Route::NS, Route::NE, Route::SN, Route::WN],
            Route::NW | Route::SE | Route::WS | Route::EN => vec![],
            Route::NE => vec![Route::EW, Route::SN, Route::SW, Route::WN, Route::ES],
            Route::SW => vec![Route::NS, Route::NE, Route::WE, Route::WN, Route::ES],
            Route::WN => vec![Route::NS, Route::NE, Route::SW, Route::EW, Route::ES],
            Route::ES => vec![Route::NE, Route::SN, Route::SW, Route::WE, Route::WN],
        }
    }
}