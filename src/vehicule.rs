use crate::config::{Direction, Route, VITESSE_MAX, VITESSE_MIN, VITESSE_NORMAL, VITESSE_NUL};
use macroquad::color::{Color, BLACK, WHITE};
use macroquad::prelude::*;
use std::time::Instant;

fn lerp_color(color1: Color, color2: Color, t: f32) -> Color {
    Color {
        r: color1.r + (color2.r - color1.r) * t,
        g: color1.g + (color2.g - color1.g) * t,
        b: color1.b + (color2.b - color1.b) * t,
        a: color1.a + (color2.a - color1.a) * t,
    }
}
#[derive(Clone)]
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
    pub width: f32,
    pub height: f32,
    pub sensor_length_large: f32,
    pub sensor_length_small: f32,
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
            width: 50.0,
            height: 23.0,
            sensor_length_large: 100.0,
            sensor_length_small: 50.0,
        }
    }

    pub fn detect_collision_large(&self, other: &Vehicule) -> bool {
        let self_front = self.get_front_point_large();
        let other_front = other.coordonne;
        // other.draw();

        let distance = self_front.distance(other_front);

        ((self_front[0] - other_front[0]).abs() <= 10.0
            || (self_front[1] - other_front[1]).abs() <= 10.0)
            && distance < self.sensor_length_large
    }

    pub fn detect_collision_small(&self, other: &Vehicule) -> bool {
        let self_front = self.get_front_point_small();
        let other_front = other.coordonne;
        // self.draw();

        let distance = self_front.distance(other_front);

        // ((self_front[0] - other_front[0]).abs() <= 10.0 ||( self_front[1] - other_front[1]).abs() <= 10.0)
        // &&
        distance < self.sensor_length_small
    }
    pub fn adjust_speed_based_on_sensors(&mut self, other: &mut Vehicule) {
        // self.draw();
        if self.detect_collision_large(other) {
            self.vitesse = VITESSE_MIN;
            other.vitesse = VITESSE_MAX;
        } else if self.detect_collision_small(other) {
            self.vitesse = VITESSE_NUL;
            other.vitesse = VITESSE_MAX;
        }
        //  else {
        // }
    }

    fn get_front_point_large(&self) -> Vec2 {
        match self.direction {
            Direction::Up => Vec2::new(
                self.coordonne.x + self.width / 2.0,
                self.coordonne.y - self.sensor_length_large,
            ),
            Direction::Down => Vec2::new(
                self.coordonne.x + self.width / 2.0,
                self.coordonne.y + self.height + self.sensor_length_large,
            ),
            Direction::Left => Vec2::new(
                self.coordonne.x - self.sensor_length_large,
                self.coordonne.y + self.height / 2.0,
            ),
            Direction::Right => Vec2::new(
                self.coordonne.x + self.width + self.sensor_length_large,
                self.coordonne.y + self.height / 2.0,
            ),
        }
    }

    fn get_front_point_small(&self) -> Vec2 {
        match self.direction {
            Direction::Up => Vec2::new(
                self.coordonne.x + self.width / 2.0,
                self.coordonne.y - self.sensor_length_small,
            ),
            Direction::Down => Vec2::new(
                self.coordonne.x + self.width / 2.0,
                self.coordonne.y + self.height + self.sensor_length_small,
            ),
            Direction::Left => Vec2::new(
                self.coordonne.x - self.sensor_length_small,
                self.coordonne.y + self.height / 2.0,
            ),
            Direction::Right => Vec2::new(
                self.coordonne.x + self.width + self.sensor_length_small,
                self.coordonne.y + self.height / 2.0,
            ),
        }
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

                if self.coordonne.x >= 592.0 {
                    self.vitesse = VITESSE_MAX;
                }
                if self.coordonne.x >= 500.0 - 140.0 && self.route == Route::WS {
                    self.set_direction(Direction::Down);
                    self.vitesse = VITESSE_MAX;
                }
            }
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

    pub fn draw(&self) {
        draw_rectangle(
            self.coordonne.x,
            self.coordonne.y,
            self.width,
            self.height,
            BLACK,
        );
    }

    pub fn draw_sensors(&self) {
        let sensor_color_large = WHITE;
        let sensor_color_small = BLACK;

        // Draw the larger sensor
        match self.direction {
            Direction::Up => {
                draw_line(
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y,
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y - self.sensor_length_large,
                    5.0,
                    lerp_color(WHITE, sensor_color_large, 0.5),
                );
            }
            Direction::Down => {
                draw_line(
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y + self.height,
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y + self.height + self.sensor_length_large,
                    5.0,
                    lerp_color(WHITE, sensor_color_large, 0.5),
                );
            }
            Direction::Left => {
                draw_line(
                    self.coordonne.x,
                    self.coordonne.y + self.height / 2.0,
                    self.coordonne.x - self.sensor_length_large,
                    self.coordonne.y + self.height / 2.0,
                    5.0,
                    lerp_color(WHITE, sensor_color_large, 0.5),
                );
            }
            Direction::Right => {
                draw_line(
                    self.coordonne.x + self.width,
                    self.coordonne.y + self.height / 2.0,
                    self.coordonne.x + self.width + self.sensor_length_large,
                    self.coordonne.y + self.height / 2.0,
                    5.0, // Thickness of 5.0 (adjust as needed)
                    lerp_color(WHITE, sensor_color_large, 0.5),
                );
            }
        }

        // Draw the smaller sensor
        match self.direction {
            Direction::Up => {
                draw_line(
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y,
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y - self.sensor_length_small,
                    2.0, // Thickness of 2.0 (adjust as needed)
                    lerp_color(WHITE, sensor_color_small, 0.5),
                );
            }
            Direction::Down => {
                draw_line(
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y + self.height,
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y + self.height + self.sensor_length_small,
                    2.0, // Thickness of 2.0 (adjust as needed)
                    lerp_color(WHITE, sensor_color_small, 0.5),
                );
            }
            Direction::Left => {
                draw_line(
                    self.coordonne.x,
                    self.coordonne.y + self.height / 2.0,
                    self.coordonne.x - self.sensor_length_small,
                    self.coordonne.y + self.height / 2.0,
                    2.0, // Thickness of 2.0 (adjust as needed)
                    lerp_color(WHITE, sensor_color_small, 0.5),
                );
            }
            Direction::Right => {
                draw_line(
                    self.coordonne.x + self.width,
                    self.coordonne.y + self.height / 2.0,
                    self.coordonne.x + self.width + self.sensor_length_small,
                    self.coordonne.y + self.height / 2.0,
                    2.0, // Thickness of 2.0 (adjust as needed)
                    lerp_color(WHITE, sensor_color_small, 0.5),
                );
            }
        }
    }
}
