use crate::intersection::*;
use crate::route::*;
use std::time::{Duration, Instant};

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
    max_time: Duration,
    min_time: Duration,
    close_calls: u32,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            passed_intersection: 0,
            max_velocity: 0.0,
            min_velocity: f32::MAX,
            max_time: Duration::from_secs(0),
            min_time: Duration::from_secs(u64::MAX),
            close_calls: 0,
        }
    }

    pub fn update(&mut self, intersection: &Intersection) {
        self.passed_intersection = intersection.number_of_passed_vehicles;
        self.max_velocity = intersection.max_velocity;
        self.min_velocity = intersection.min_velocity;
        self.close_calls = intersection.close_calls;
        self.max_time = intersection.max_time;
        self.min_time = intersection.min_time;
    }
}

pub enum GameState {
    Game,
    Statistics,
}

/*
Pour gere la taille de la fenaitre
*/
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
    let throttle_duration = Duration::from_millis(200); // 200 milliseconds throttle
    let mut last_key_press = Instant::now() - throttle_duration; // Ensure first key press isn't throttled

    loop {
        match game_state {
            GameState::Game => {
                clear_background(WHITE);
                draw_texture(&img, 0.0, 0.0, WHITE);

                intersection.drive_cars();
                intersection.remove_cars();
                intersection.draw_cars(car_1.clone(), car_2.clone(), car_3.clone());

                if last_key_press.elapsed() >= throttle_duration {
                    if is_key_pressed(KeyCode::Left) {
                        let routes = vec![Route::EW, Route::EN, Route::ES];
                        intersection.add_car(routes.clone(), cars.clone());
                        last_key_press = Instant::now();
                    }

                    if is_key_pressed(KeyCode::Right) {
                        let routes = vec![Route::WE, Route::WS, Route::WN];
                        intersection.add_car(routes.clone(), cars.clone());
                        last_key_press = Instant::now();
                    }

                    if is_key_pressed(KeyCode::Up) {
                        let routes = vec![Route::SN, Route::SE, Route::SW];
                        intersection.add_car(routes.clone(), cars.clone());
                        last_key_press = Instant::now();
                    }

                    if is_key_pressed(KeyCode::Down) {
                        let routes = vec![Route::NS, Route::NW, Route::NE];
                        intersection.add_car(routes.clone(), cars.clone());
                        last_key_press = Instant::now();
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
                        last_key_press = Instant::now();
                    }

                    if is_key_pressed(KeyCode::Escape) {
                        statistics.update(&intersection);
                        game_state = GameState::Statistics;
                        window.set_size(500, 400);
                        window.set_title("Simulation Statistics");
                        last_key_press = Instant::now();
                    }
                }
            }
            GameState::Statistics => {
                clear_background(WHITE);
                draw_stats_text(
                    &format!("Vehicles passed: {}", statistics.passed_intersection),
                    50.0,
                );
                draw_stats_text(
                    &format!("Max velocity: {:.1} ", statistics.max_velocity),
                    100.0,
                );
                draw_stats_text(
                    &format!("Min velocity: {:.1} ", statistics.min_velocity),
                    150.0,
                );
                draw_stats_text(
                    &format!("Max time: {:.2} s", statistics.max_time.as_secs_f32()),
                    200.0,
                );
                draw_stats_text(
                    &format!("Min time: {:.2} s", statistics.min_time.as_secs_f32()),
                    250.0,
                );
                draw_stats_text(&format!("Close calls: {}", statistics.close_calls), 300.0);
                draw_stats_text("Press SPACE to quit", 350.0);

                if is_key_pressed(KeyCode::Space) {
                    std::process::exit(0);
                }
            }
        }

        next_frame().await;
    }
}