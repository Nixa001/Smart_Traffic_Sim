use macroquad::prelude::*;
use crate::{Route, Direction, constants::*};

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

    fn before_cross_road(&self) -> bool {
        match self.direction {
            Direction::Right => self.coordonne.x < AVANT_INTERSECTION.x,
            Direction::Left => self.coordonne.x > AVANT_INTERSECTION.y,
            Direction::Down => self.coordonne.y < AVANT_INTERSECTION.x,
            Direction::Up => self.coordonne.y > AVANT_INTERSECTION.y,
        }
    }

    fn in_stop_zone(&self) -> bool {
        return match self.direction {
            Direction::Right => self.coordonne.x > APRES_INTERSECTION.x - CAR_WIDTH,
            Direction::Left => self.coordonne.x < APRES_INTERSECTION.y,
            Direction::Down => self.coordonne.y > APRES_INTERSECTION.x - CAR_WIDTH,
            Direction::Up => self.coordonne.y < APRES_INTERSECTION.y,
        };
    }

    fn after_cross_road(&self) -> bool {
        match self.direction {
            Direction::Right => self.coordonne.x > APRES_INTERSECTION.y,
            Direction::Left => self.coordonne.x < APRES_INTERSECTION.x,
            Direction::Down => self.coordonne.y > APRES_INTERSECTION.y,
            Direction::Up => self.coordonne.y < APRES_INTERSECTION.x,
        }
    }

    fn on_cross_road(&self) -> bool {
        return !self.before_cross_road() && !self.after_cross_road();
    }

    fn speed_up(&mut self) {
        self.vitesse = match self.direction {
            Direction::Down => (0.0, VITESSE_RAPID),
            Direction::Up => (0.0, -VITESSE_RAPID),
            Direction::Right => (VITESSE_RAPID, 0.0),
            Direction::Left => (-VITESSE_RAPID, 0.0),
        }
    }

    fn is_speed_up(&self) -> bool {
        return self.vitesse.0.abs() == VITESSE_RAPID || self.vitesse.1.abs() == VITESSE_RAPID;
    }

    fn is_slow_down(&self) -> bool {
        return self.vitesse.0.abs() == VITESSE_MIN || self.vitesse.1.abs() == VITESSE_MIN;
    }

    fn slow_down(&mut self) {
        self.vitesse = match self.direction {
            Direction::Down => (0.0, VITESSE_MIN),
            Direction::Up => (0.0, -VITESSE_MIN),
            Direction::Right => (VITESSE_MIN, 0.0),
            Direction::Left => (-VITESSE_MIN, 0.0),
        }
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

    fn turn(&mut self) {
        let vitesse = self.vitesse;
        let r = self.rectangle;
        self.rectangle.0 = r.1;
        self.rectangle.1 = r.0;
        self.turned = true;

        match self.route {
            Route::NE => {
                self.vitesse.0 = vitesse.1;
                self.vitesse.1 = vitesse.0;
                self.direction = Direction::Right;
                self.coordonne.y = 515.0;
                self.rotation = 0.0;
            }
            Route::SW => {
                self.vitesse.0 = vitesse.1;
                self.vitesse.1 = vitesse.0;
                self.direction = Direction::Left;
                self.coordonne.y = 475.0;
                self.rotation = 180.0;
            }
            Route::WN => {
                self.vitesse.0 = -vitesse.1;
                self.vitesse.1 = -vitesse.0;
                self.direction = Direction::Up;
                self.coordonne.x = 500.0;
                self.rotation = 270.0;
            }
            Route::ES => {
                self.vitesse.0 = -vitesse.1;
                self.vitesse.1 = -vitesse.0;
                self.direction = Direction::Down;
                self.coordonne.x = 455.0;
                self.rotation = 90.0;
            }
            Route::NW => {
                self.vitesse.0 = -vitesse.1;
                self.vitesse.1 = -vitesse.0;
                self.direction = Direction::Left;
                self.coordonne.y = 370.0;
                self.rotation = 180.0;
            }
            Route::SE => {
                self.vitesse.0 = -vitesse.1;
                self.vitesse.1 = -vitesse.0;
                self.coordonne.y = 610.0;
                self.direction = Direction::Right;
                self.rotation = 0.0;
            }
            Route::WS => {
                self.direction = Direction::Down;
                self.vitesse.0 = vitesse.1;
                self.vitesse.1 = vitesse.0;
                self.coordonne.x = 360.0;
                self.rotation = 90.0;
            }
            Route::EN => {
                self.direction = Direction::Up;
                self.vitesse.0 = vitesse.1;
                self.vitesse.1 = vitesse.0;
                self.coordonne.x = 600.0;
                self.rotation = 270.0;
            }
            _ => return,
        }
    }

    pub fn drive_away(&self) -> bool {
        return match self.direction {
            Direction::Right => self.coordonne.x > 1000.0,
            Direction::Left => self.coordonne.x < 0.0 - CAR_WIDTH,
            Direction::Down => self.coordonne.y > 1000.0,
            Direction::Up => self.coordonne.y < 0.0 - CAR_WIDTH,
        };
    }
}