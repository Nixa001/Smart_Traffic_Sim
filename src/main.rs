use crate::constants::*;
use crate::intersection::*;
use crate::route::*;
use std::time::Instant;

use macroquad::prelude::*;

mod constants;
mod intersection;
mod route;
mod vehicule;

struct Window {
    width: i32,
    height: i32,
    title: String,
}

impl Window {
    fn new(width: i32, height: i32, title: &str) -> Self {
        Self {
            width,
            height,
            title: title.to_string(),
        }
    }

    fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
}

pub fn draw_stats_text(text: &str, y_offset: f32) {
    let dims = measure_text(text, None, 30, 1.0);
    draw_text_ex(
        text,
        screen_width() / 2.0 - dims.width / 2.0,
        y_offset,
        TextParams {
            font_size: 30,
            color: BLACK,
            ..Default::default()
        },
    );
}

pub struct Statistics {
    passed_intersection: u32,
    max_velocity: f32,
    min_velocity: f32,
    max_time: f32,
    min_time: f32,
    close_calls: u32,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            passed_intersection: 0,
            max_velocity: 0.0,
            min_velocity: f32::MAX,
            max_time: 0.0,
            min_time: f32::MAX,
            close_calls: 0,
        }
    }

    pub fn update(&mut self, intersection: &Intersection) {
        self.passed_intersection = intersection.number_of_passed_vehicles;
        self.max_velocity = intersection.max_velocity;
        self.min_velocity = intersection.min_velocity;
        self.close_calls = intersection.close_calls;
    }
}

pub enum GameState {
    Game,
    Statistics,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "IBG Road".to_owned(),
        window_width: 1000,
        window_height: 1000,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]

async fn main() {
    let mut window = Window::new(500, 500, "IBG Road");
    let img: Texture2D = load_texture("./assets/road.png").await.unwrap();
    let car_1: Texture2D = load_texture("./assets/car_blue.png").await.unwrap();
    let car_2: Texture2D = load_texture("./assets/car_green.png").await.unwrap();
    let car_3: Texture2D = load_texture("./assets/car_white.png").await.unwrap();
    let mut game_state = GameState::Game;
    let mut statistics = Statistics::new();
    let mut intersection = Intersection::new();

    let cars: Vec<Texture2D> = vec![car_1.clone(), car_2.clone(), car_3.clone()];
    let start_time = Instant::now();

    loop {
        match game_state {
            GameState::Game => {
                clear_background(WHITE);
                draw_texture(&img, 0.0, 0.0, WHITE);

                intersection.drive_cars();
                intersection.remove_cars();
                intersection.draw_cars(car_1.clone(), car_2.clone(), car_3.clone());

                if is_key_pressed(KeyCode::Left) {
                    let routes = vec![Route::EW, Route::EN, Route::ES];
                    intersection.add_car(routes.clone(), cars.clone());
                }

                if is_key_pressed(KeyCode::Right) {
                    let routes = vec![Route::WE, Route::WS, Route::WN];
                    intersection.add_car(routes.clone(), cars.clone());
                }

                if is_key_pressed(KeyCode::Up) {
                    let routes = vec![Route::SN, Route::SE, Route::SW];
                    intersection.add_car(routes.clone(), cars.clone());
                }

                if is_key_pressed(KeyCode::Down) {
                    let routes = vec![Route::NS, Route::NW, Route::NE];
                    intersection.add_car(routes.clone(), cars.clone());
                }

                if is_key_pressed(KeyCode::R) {
                    let routes = vec![
                        Route::EW,
                        Route::WE,
                        Route::SN,
                        Route::NS,
                        Route::EN,
                        Route::WS,
                        Route::NW,
                        Route::SE,
                        Route::NE,
                        Route::SW,
                        Route::WN,
                        Route::ES,
                    ];
                    intersection.add_car(routes.clone(), cars.clone());
                }

                if is_key_pressed(KeyCode::Escape) {
                    statistics.update(&intersection);
                    let elapsed = start_time.elapsed().as_secs_f32();
                    statistics.max_time = statistics.max_time.max(elapsed);
                    statistics.min_time = statistics.min_time.min(elapsed);
                    game_state = GameState::Statistics;
                    window.set_size(500, 400);
                    window.set_title("Statistiques de simulation");
                }
            }
            GameState::Statistics => {
                clear_background(WHITE);
                draw_stats_text(
                    &format!("Véhicules passés: {}", statistics.passed_intersection),
                    50.0,
                );
                draw_stats_text(
                    &format!("Vitesse max: {:.2} px/s", statistics.max_velocity),
                    100.0,
                );
                draw_stats_text(
                    &format!("Vitesse min: {:.2} px/s", statistics.min_velocity),
                    150.0,
                );
                draw_stats_text(&format!("Temps max: {:.2} s", statistics.max_time), 200.0);
                draw_stats_text(&format!("Temps min: {:.2} s", statistics.min_time), 250.0);
                draw_stats_text(
                    &format!("Appels proches: {}", statistics.close_calls),
                    300.0,
                );
                draw_stats_text("Appuyez sur ESPACE pour quitter", 350.0);

                if is_key_pressed(KeyCode::Space) {
                    std::process::exit(0);
                }
            }
        }

        next_frame().await;
    }
}
