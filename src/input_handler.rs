use crate::config::{Direction, Route, VITESSE_NORMAL};

use crate::vehicule::Vehicule;
use ::rand::Rng;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

pub struct InputHandler {
    last_key_press_time: Instant,
    cooldown: Duration,
}

impl InputHandler {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            last_key_press_time: Instant::now() - cooldown,
            cooldown,
        }
    }

    pub fn handle_input(
        &mut self,
        vehicules: &mut Vec<Vehicule>,
        next_id: &mut i32,
        rng: &mut impl Rng,
    ) {
        if self.last_key_press_time.elapsed() < self.cooldown {
            return;
        }

        let possible_x_values = [500.0, 548.0, 592.0];
        let possible_y_values = [515.0, 560.0, 610.0];
        let routes = [
            Route::NE,
            Route::NS,
            Route::NW,
            Route::SN,
            Route::SE,
            Route::SW,
            Route::EN,
            Route::EW,
            Route::ES,
            Route::WE,
            Route::WS,
            Route::WN,
        ];
        let random_x = possible_x_values[rng.gen_range(0..possible_x_values.len())];
        let random_y = possible_y_values[rng.gen_range(0..possible_y_values.len())];
        let mut random_route = routes[rng.gen_range(0..routes.len())];

        let (coordonne, direction) = match random_route {
            Route::NE => (vec2(592.0 - 140.0, 0.0), Direction::Down),
            Route::NS => (vec2(548.0 - 140.0, 0.0), Direction::Down),
            Route::NW => (vec2(500.0 - 140.0, 0.0), Direction::Down),
            Route::SN => (vec2(548.0, 990.0), Direction::Up),
            Route::SE => (vec2(592.0, 990.0), Direction::Up),
            Route::SW => (vec2(500.0, 990.0), Direction::Up),
            Route::EN => (vec2(950.0, 515.0 - 140.0), Direction::Left),
            Route::EW => (vec2(950.0, 560.0 - 140.0), Direction::Left),
            Route::ES => (vec2(950.0, 610.0 - 140.0), Direction::Left),
            Route::WE => (vec2(0.0, 560.0), Direction::Right),
            Route::WS => (vec2(0.0, 610.0), Direction::Right),
            Route::WN => (vec2(0.0, 515.0), Direction::Right),
        };

        if is_key_pressed(KeyCode::Up) {
            random_route = match (random_x, 990.0) {
                (548.0, 990.0) => Route::SN,
                (592.0, 990.0) => Route::SE,
                (500.0, 990.0) => Route::SW,
                _ => todo!(),
            };
            vehicules.push(Vehicule::new(
                *next_id,
                vec2(random_x, 990.0),
                VITESSE_NORMAL,
                Direction::Up,
                random_route,
            ));
            *next_id += 1;
            self.last_key_press_time = Instant::now();
        }
        if is_key_pressed(KeyCode::Down) {
            random_route = match (random_x, 0.0) {
                (548.0, 0.0) => Route::NS,
                (592.0, 0.0) => Route::NE,
                (500.0, 0.0) => Route::NW,
                _ => todo!(),
            };
            vehicules.push(Vehicule::new(
                *next_id,
                vec2(random_x - 140.0, 0.0),
                VITESSE_NORMAL,
                Direction::Down,
                random_route,
            ));
            *next_id += 1;
            self.last_key_press_time = Instant::now();
        }
        if is_key_pressed(KeyCode::Left) {
            random_route = match (950.0, random_y) {
                (950.0, 515.0) => Route::EN,
                (950.0, 560.0) => Route::EW,
                (950.0, 610.0) => Route::ES,
                _ => todo!(),
            };
            vehicules.push(Vehicule::new(
                *next_id,
                vec2(950.0, random_y - 140.0),
                VITESSE_NORMAL,
                Direction::Left,
                random_route,
            ));
            *next_id += 1;
            self.last_key_press_time = Instant::now();
        }
        if is_key_pressed(KeyCode::Right) {
            random_route = match (0.0, random_y) {
                (0.0, 515.0) => Route::WN,
                (0.0, 560.0) => Route::WE,
                (0.0, 610.0) => Route::WS,
                _ => todo!(),
            };
            vehicules.push(Vehicule::new(
                *next_id,
                vec2(0.0, random_y),
                VITESSE_NORMAL,
                Direction::Right,
                random_route,
            ));
            *next_id += 1;
            self.last_key_press_time = Instant::now();
        }
        if is_key_pressed(KeyCode::R) {
            vehicules.push(Vehicule::new(
                *next_id,
                coordonne,
                VITESSE_NORMAL,
                direction,
                random_route,
            ));
            *next_id += 1;
            self.last_key_press_time = Instant::now();
        }
    }
}




// pub struct InputHandler {
//     last_key_press_time: Instant,
//     cooldown: Duration,
// }

// impl InputHandler {
//     pub fn new(cooldown: Duration) -> Self {
//         Self {
//             last_key_press_time: Instant::now() - cooldown,
//             cooldown,
//         }
//     }

//     pub fn handle_input(
//         &mut self,
//         vehicules: &mut Vec<Vehicule>,
//         next_id: &mut i32,
//         rng: &mut impl Rng,
//     ) {
//         if self.last_key_press_time.elapsed() < self.cooldown {
//             return;
//         }

//         let possible_x_values = [500.0, 548.0, 592.0];
//         let possible_y_values = [515.0, 560.0, 610.0];
//         let routes = [
//             Route::NE,
//             Route::NS,
//             Route::NW,
//             Route::SN,
//             Route::SE,
//             Route::SW,
//             Route::EN,
//             Route::EW,
//             Route::ES,
//             Route::WE,
//             Route::WS,
//             Route::WN,
//         ];
//         let random_x = possible_x_values[rng.gen_range(0..possible_x_values.len())];
//         let random_y = possible_y_values[rng.gen_range(0..possible_y_values.len())];
//         let mut random_route = routes[rng.gen_range(0..routes.len())];

//         let (coordonne, direction) = match random_route {
//             Route::NE => (vec2(592.0 - 140.0, 0.0), Direction::Down),
//             Route::NS => (vec2(548.0 - 140.0, 0.0), Direction::Down),
//             Route::NW => (vec2(500.0 - 140.0, 0.0), Direction::Down),
//             Route::SN => (vec2(548.0, 990.0), Direction::Up),
//             Route::SE => (vec2(592.0, 990.0), Direction::Up),
//             Route::SW => (vec2(500.0, 990.0), Direction::Up),
//             Route::EN => (vec2(950.0, 515.0 - 140.0), Direction::Left),
//             Route::EW => (vec2(950.0, 560.0 - 140.0), Direction::Left),
//             Route::ES => (vec2(950.0, 610.0 - 140.0), Direction::Left),
//             Route::WE => (vec2(0.0, 560.0), Direction::Right),
//             Route::WS => (vec2(0.0, 610.0), Direction::Right),
//             Route::WN => (vec2(0.0, 515.0), Direction::Right),
//         };

//         if is_key_pressed(KeyCode::Up) {
//             random_route = match (random_x, 990.0) {
//                 (548.0, 990.0) => Route::SN,
//                 (592.0, 990.0) => Route::SE,
//                 (500.0, 990.0) => Route::SW,
//                 _ => todo!(),
//             };
//             vehicules.push(Vehicule::new(
//                 *next_id,
//                 vec2(random_x, 990.0),
//                 VITESSE_NORMAL,
//                 Direction::Up,
//                 random_route,
//             ));
//             *next_id += 1;
//             self.last_key_press_time = Instant::now();
//         }
//         if is_key_pressed(KeyCode::Down) {
//             random_route = match (random_x, 0.0) {
//                 (548.0, 0.0) => Route::NS,
//                 (592.0, 0.0) => Route::NE,
//                 (500.0, 0.0) => Route::NW,
//                 _ => todo!(),
//             };
//             vehicules.push(Vehicule::new(
//                 *next_id,
//                 vec2(random_x - 140.0, 0.0),
//                 VITESSE_NORMAL,
//                 Direction::Down,
//                 random_route,
//             ));
//             *next_id += 1;
//             self.last_key_press_time = Instant::now();
//         }
//         if is_key_pressed(KeyCode::Left) {
//             random_route = match (950.0, random_y) {
//                 (950.0, 515.0) => Route::EN,
//                 (950.0, 560.0) => Route::EW,
//                 (950.0, 610.0) => Route::ES,
//                 _ => todo!(),
//             };
//             vehicules.push(Vehicule::new(
//                 *next_id,
//                 vec2(950.0, random_y - 140.0),
//                 VITESSE_NORMAL,
//                 Direction::Left,
//                 random_route,
//             ));
//             *next_id += 1;
//             self.last_key_press_time = Instant::now();
//         }
//         if is_key_pressed(KeyCode::Right) {
//             random_route = match (0.0, random_y) {
//                 (0.0, 515.0) => Route::WN,
//                 (0.0, 560.0) => Route::WE,
//                 (0.0, 610.0) => Route::WS,
//                 _ => todo!(),
//             };
//             vehicules.push(Vehicule::new(
//                 *next_id,
//                 vec2(0.0, random_y),
//                 VITESSE_NORMAL,
//                 Direction::Right,
//                 random_route,
//             ));
//             *next_id += 1;
//             self.last_key_press_time = Instant::now();
//         }
//         if is_key_pressed(KeyCode::R) {
//             vehicules.push(Vehicule::new(
//                 *next_id,
//                 coordonne,
//                 VITESSE_NORMAL,
//                 direction,
//                 random_route,
//             ));
//             *next_id += 1;
//             self.last_key_press_time = Instant::now();
//         }
//     }
// }

// pub const VITESSE_NUL: f32 = 0.0;
// pub const VITESSE_MIN: f32 = 0.2;
// pub const VITESSE_NORMAL: f32 = 0.7;
// pub const VITESSE_MAX: f32 = 1.2;

// #[derive(PartialEq, Copy, Clone)]
// pub enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }
// #[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
// pub enum Route {
//     NE,
//     NS,
//     NW,
//     SN,
//     SE,
//     SW,
//     EN,
//     EW,
//     ES,
//     WE,
//     WS,
//     WN,
// }
