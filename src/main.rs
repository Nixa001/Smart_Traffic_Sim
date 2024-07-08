use ::rand::Rng;
use macroquad::prelude::*;
use std::time::Duration;

mod config;
mod input_handler;
mod vehicule;

fn window_conf() -> Conf {
    Conf {
        window_title: "IBG Road".to_owned(),
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let img: Texture2D = load_texture("./assets/road.png").await.unwrap();
    let car_1: Texture2D = load_texture("./assets/car_blue.png").await.unwrap();
    let car_2: Texture2D = load_texture("./assets/car_green.png").await.unwrap();
    let car_3: Texture2D = load_texture("./assets/car_white.png").await.unwrap();

    let mut rng = ::rand::thread_rng();

    let mut vehicules: Vec<Vehicule> = Vec::new();
    let mut next_id = 1;
    let mut input_handler = InputHandler::new(Duration::from_millis(400));

    loop {
        input_handler.handle_input(&mut vehicules, &mut next_id, &mut rng);

        clear_background(WHITE);
        draw_texture(&img, 0.0, 0.0, WHITE);

        let delta_time = get_frame_time();

        // Mise à jour et gestion des collisions
        for i in 0..vehicules.len() {
            let mut current_vehicle = vehicules[i].clone();
            current_vehicle.update(delta_time);

            vehicules[i] = current_vehicle;
        }

        // Dessin des véhicules et capteurs
        for vehicule in &vehicules {
            let draw_params = DrawTextureParams {
                dest_size: Some(Vec2::new(vehicule.width, vehicule.height)),
                rotation: vehicule.rotation.to_radians(),
                ..Default::default()
            };

            let car = match vehicule.route {
                Route::SE | Route::EN | Route::NW | Route::WS => &car_1,
                Route::NS | Route::SN | Route::WE | Route::EW => &car_2,
                _ => &car_3,
            };

            draw_texture_ex(
                car,
                vehicule.coordonne.x,
                vehicule.coordonne.y,
                WHITE,
                draw_params,
            );

            vehicule.draw_sensors();
        }

        vehicules.retain(|v| {
            v.coordonne.x > -100.0
                && v.coordonne.x < 1100.0
                && v.coordonne.y > -100.0
                && v.coordonne.y < 1100.0
        });

        // let vehicles_count = vehicules.len();
        // draw_text(
        //     &format!("Vehicles: {}", vehicles_count),
        //     10.0,
        //     20.0,
        //     20.0,
        //     BLACK,
        // );

        next_frame().await;
    }
}

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
            sensor_length_small: 30.0,
        }
    }

    pub fn detect_collision_large(&self, other: &Vehicule) -> bool {
        let self_front = self.get_front_point_large();
        let other_front = other.get_front_point_large();

        let distance = self_front.distance(other_front);
        let safe_distance = 100.0;

        distance < safe_distance
    }

    pub fn detect_collision_small(&self, other: &Vehicule) -> bool {
        let self_front = self.get_front_point_small();
        let other_front = other.get_front_point_small();

        let distance = self_front.distance(other_front);
        let safe_distance = 30.0;

        distance < safe_distance
    }
    pub fn adjust_speed_based_on_sensors(&mut self, other: &mut Vehicule) {
        if self.detect_collision_large(other) {
            self.vitesse = VITESSE_MIN;
            other.vitesse = VITESSE_MAX;
        } else if self.detect_collision_small(other) {
            self.vitesse = VITESSE_NUL;
        }
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
                    5.0, // Thickness of 5.0 (adjust as needed)
                    lerp_color(WHITE, sensor_color_large, 0.5),
                );
            }
            Direction::Down => {
                draw_line(
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y + self.height,
                    self.coordonne.x + self.width / 2.0,
                    self.coordonne.y + self.height + self.sensor_length_large,
                    5.0, // Thickness of 5.0 (adjust as needed)
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

pub const VITESSE_NUL: f32 = 0.0;
pub const VITESSE_MIN: f32 = 0.2;
pub const VITESSE_NORMAL: f32 = 0.7;
pub const VITESSE_MAX: f32 = 1.2;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Route {
    NE,
    NS,
    NW,
    SN,
    SE,
    SW,
    EN,
    EW,
    ES,
    WE,
    WS,
    WN,
}
