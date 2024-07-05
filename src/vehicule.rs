use crate::config::{Direction, Route, VITESSE_MAX, VITESSE_MIN, VITESSE_NORMAL};
use macroquad::prelude::*;
use std::time::Instant;

pub struct Vehicule {
    pub id: i32,
    pub coordonne: Vec2,
    pub vitesse: f32,
    pub direction: Direction,
    pub rotation: f32,
    pub route: Route,
    pub time: f32,
    pub distance: f32,
    pub start_time: Instant,
}

impl Vehicule {
    pub fn new(id: i32, coordonne: Vec2, vitesse: f32, direction: Direction, route: Route) -> Self {
        let rotation = match direction {
            Direction::Up => -90.0,
            Direction::Down => 90.0,
            Direction::Left => 180.0,
            Direction::Right => 0.0,
        };
        Vehicule {
            id,
            coordonne,
            vitesse,
            direction,
            rotation,
            route,
            time: 0.0,
            distance: 0.0,
            start_time: Instant::now(),
        }
    }
    pub fn detect_collision(&self, other: &Vehicule, sensor_distance: f32) -> bool {
        let (sensor_x, sensor_y) = match self.direction {
            Direction::Up => (self.coordonne.x, self.coordonne.y - sensor_distance),
            Direction::Down => (self.coordonne.x, self.coordonne.y + sensor_distance),
            Direction::Left => (self.coordonne.x - sensor_distance, self.coordonne.y),
            Direction::Right => (self.coordonne.x + sensor_distance, self.coordonne.y),
        };

        let dist_x = (sensor_x - other.coordonne.x).abs();
        let dist_y = (sensor_y - other.coordonne.y).abs();

        let collision_threshold = 50.0;

        dist_x < collision_threshold && dist_y < collision_threshold
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
        self.distance += self.vitesse * delta_time;

        match self.direction {
            Direction::Up => {
                self.coordonne.y -= self.vitesse;
                if self.coordonne.y <= 610.0 && self.route == Route::SE {
                    self.set_direction(Direction::Right);
                    self.vitesse = VITESSE_MAX;
                }
                
                if self.coordonne.y <= 515.0 - 140.0 {
                    self.vitesse = VITESSE_MAX;
                }
                if self.coordonne.y <= 610.0 - 140.0 && self.route == Route::SW {
                    self.set_direction(Direction::Left);
                }
            }
            Direction::Down => {
                self.coordonne.y += self.vitesse;
                if self.coordonne.y >= 515.0 && self.route == Route::NE {
                    self.set_direction(Direction::Right);
                }
                
                if self.coordonne.y >= 610.0 - 140.0 {
                    self.vitesse = VITESSE_MAX;
                }
                if self.coordonne.y >= 515.0 - 140.0 && self.route == Route::NW {
                    self.set_direction(Direction::Left);
                    self.vitesse = VITESSE_MAX;
                }
            }
            Direction::Left => {
                self.coordonne.x -= self.vitesse;
                if self.coordonne.x <= 592.0 && self.route == Route::EN {
                    self.set_direction(Direction::Up);
                    self.vitesse = VITESSE_MAX;
                }
                if self.coordonne.x <= 500.0 - 140.0 {
                    self.vitesse = VITESSE_MAX;
                }
                if self.coordonne.x <= 592.0 - 140.0 && self.route == Route::ES {
                    self.set_direction(Direction::Down);
                }
            }
            Direction::Right => {
                self.coordonne.x += self.vitesse;
                if self.coordonne.x >= 500.0 && self.route == Route::WN {
                    self.set_direction(Direction::Up);
                }

                if self.coordonne.x >= 592.0{
                    self.vitesse = VITESSE_MAX;
                }
                if self.coordonne.x >= 500.0 - 140.0 && self.route == Route::WS {
                    self.set_direction(Direction::Down);
                    self.vitesse = VITESSE_MAX;
                }
            }
        }

        match self.route {
            _ => (),
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.rotation = match direction {
            Direction::Up => -90.0,
            Direction::Down => 90.0,
            Direction::Left => 180.0,
            Direction::Right => 0.0,
        };
    }
}
