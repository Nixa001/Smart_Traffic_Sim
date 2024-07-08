use macroquad::prelude::*;
use crate::constants::*;
use crate::route::Route;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(PartialEq)]
pub enum Turning {
    Left,
    Right,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vehicule {
    pub id: u32,
    pub color: Texture2D,
    pub coordonne: Vec2,
    pub vitesse: (f32, f32),
    pub rotation: f32,
    pub rectangle: (f32, f32),
    pub direction: Direction,
    pub route: Route,
    pub turned: bool,
}

impl Vehicule {
    pub fn new(
        coordonne: Vec2,
        rectangle: (f32, f32),
        color: Texture2D,
        vitesse: (f32, f32),
        id: u32,
        direction: Direction,
        route: Route,
        turned: bool,
    ) -> Vehicule {
        let rotation = match direction {
            Direction::Up => -90.0,
            Direction::Down => 90.0,
            Direction::Left => 180.0,
            Direction::Right => 0.0,
        };
        Vehicule {
            color,
            rectangle,
            coordonne,
            vitesse,
            rotation,
            id,
            direction,
            route,
            turned,
        }
    }

    pub fn drive(&mut self) {
        if self.on_turn_point() && !self.turned {
            self.turn();
        }
        self.coordonne = vec2(
            self.coordonne.x + self.vitesse.0,
            self.coordonne.y + self.vitesse.1,
        );
    }

    fn on_turn_point(&self) -> bool {
        return match self.route {
            Route::NW => self.coordonne.y >= 340.0,
            Route::SE => self.coordonne.y <= 625.0,
            Route::WS => self.coordonne.x > 375.0,
            Route::EN => self.coordonne.x < 625.0,

            Route::NE => self.coordonne.y > 515.0,
            Route::SW => self.coordonne.y < 485.0,
            Route::WN => self.coordonne.x > 515.0,
            Route::ES => self.coordonne.x < 485.0,
            _ => false,
        };
    }
    pub fn draw(&self, car1: Texture2D, car2: Texture2D, car3: Texture2D) {
        let draw_params = DrawTextureParams {
            dest_size: Some(Vec2::new(CAR_WIDTH, CAR_HEIGHT)),
            rotation: self.rotation.to_radians(),
            ..Default::default()
        };

        let car = match self.route {
            Route::SE | Route::EN | Route::NW | Route::WS => &car1,
            Route::NS | Route::SN | Route::WE | Route::EW => &car2,
            _ => &car3,
        };

        draw_texture_ex(car, self.coordonne.x, self.coordonne.y, WHITE, draw_params);
    }
    fn get_coordinates(&self) -> Vec2 {
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
    fn get_speed(&self) -> (f32, f32) {
        match *self {
            Route::NS => (0.0, VITESSE_NORMAL),
            Route::NW => (0.0, VITESSE_NORMAL),
            Route::NE => (0.0, VITESSE_NORMAL),

            Route::SN => (0.0, -VITESSE_NORMAL),
            Route::SE => (0.0, -VITESSE_NORMAL),
            Route::SW => (0.0, -VITESSE_NORMAL),

            Route::WE => (VITESSE_NORMAL, 0.0),
            Route::WS => (VITESSE_NORMAL, 0.0),
            Route::WN => (VITESSE_NORMAL, 0.0),

            Route::EW => (-VITESSE_NORMAL, 0.0),
            Route::EN => (-VITESSE_NORMAL, 0.0),
            Route::ES => (-VITESSE_NORMAL, 0.0),
        }
    }
    fn get_direction(&self) -> Direction {
        match *self {
            Route::NS => Direction::Down,
            Route::NW => Direction::Down,
            Route::NE => Direction::Down,

            Route::SN => Direction::Up,
            Route::SE => Direction::Up,
            Route::SW => Direction::Up,

            Route::WE => Direction::Right,
            Route::WS => Direction::Right,
            Route::WN => Direction::Right,

            Route::EW => Direction::Left,
            Route::EN => Direction::Left,
            Route::ES => Direction::Left,
        }
    }

    fn not_allowed_to_go(&self) -> Vec<Route> {
        match *self {
            Route::NS => vec![Route::EW, Route::WE, Route::WN, Route::SW],
            Route::SN => vec![Route::NE, Route::WE, Route::ES, Route::EW],
            Route::WE => vec![Route::NS, Route::SW, Route::SN, Route::ES],
            Route::EW => vec![Route::NS, Route::NE, Route::SN, Route::WN],

            Route::NW => vec![],
            Route::SE => vec![],
            Route::WS => vec![],
            Route::EN => vec![],

            Route::NE => vec![Route::EW, Route::SN, Route::SW, Route::WN, Route::ES],
            Route::SW => vec![Route::NS, Route::NE, Route::WE, Route::WN, Route::ES],
            Route::WN => vec![Route::NS, Route::NE, Route::SW, Route::EW, Route::ES],
            Route::ES => vec![Route::NE, Route::SN, Route::SW, Route::WE, Route::WN],
        }
    }

    // Implement other methods (before_cross_road, in_stop_zone, after_cross_road, on_cross_road, speed_up, is_speed_up, is_slow_down, slow_down, on_turn_point, turn, drive_away)
}