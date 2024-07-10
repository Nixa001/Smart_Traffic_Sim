use crate::constants::*;
use crate::intersection::*;
use crate::route::*;
use macroquad::prelude::*;

mod constants;
mod intersection;
mod route;
mod vehicule;

fn window_conf() -> Conf {
    Conf {
        window_title: "IBG Road".to_owned(),
        window_width: 1000,
        window_height: 1000,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw_intersection_rectangle() {
    // Définir les coordonnées du rectangle de l'intersection
    let left = AVANT_INTERSECTION.x;
    let right = APRES_INTERSECTION.y;
    let top = APRES_INTERSECTION.y;
    let bottom = AVANT_INTERSECTION.x;

    // Dessiner les lignes du rectangle
    draw_line(left, top, right, top, 2.0, RED); // Ligne supérieure
    draw_line(left, bottom, right, bottom, 2.0, RED); // Ligne inférieure
    draw_line(left, top, left, bottom, 2.0, RED); // Ligne gauche
    draw_line(right, top, right, bottom, 2.0, RED); // Ligne droite

    // Dessiner les lignes d'arrêt
    let stop_right = APRES_INTERSECTION.x;
    let stop_bottom = APRES_INTERSECTION.x;
    draw_line(stop_right, top, stop_right, bottom, 2.0, YELLOW); // Ligne d'arrêt droite
    draw_line(left, stop_bottom, right, stop_bottom, 2.0, YELLOW); // Ligne d'arrêt inférieure
}

#[macroquad::main(window_conf)]
async fn main() {
    let img: Texture2D = load_texture("./assets/road.png").await.unwrap();
    let car_1: Texture2D = load_texture("./assets/car_blue.png").await.unwrap();
    let car_2: Texture2D = load_texture("./assets/car_green.png").await.unwrap();
    let car_3: Texture2D = load_texture("./assets/car_white.png").await.unwrap();
    let mut game_state = GameState::Menu;
    let mut statistics = Statistics::new();
    let mut intersection = Intersection::new();

    let cars: Vec<Texture2D> = vec![car_1.clone(), car_2.clone(), car_3.clone()];
    loop {
        clear_background(WHITE);
        draw_texture(&img, 0.0, 0.0, WHITE);
        draw_intersection_rectangle();

        match game_state {
            GameState::Menu => {
                game_state = GameState::Game;
            }
            GameState::Game => {
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
                    game_state = GameState::Statistics;
                }
            }
            GameState::Statistics => {
                statistics.passed_intersection = intersection.number_of_passed_vehicles;
                draw_title_text(&format!(
                    "STATISTICS: cars finished: {}",
                    statistics.passed_intersection
                ));
            }
        }

        next_frame().await;
    }
}

pub fn draw_title_text(text: &str) {
    let dims = measure_text(text, Default::default(), 50u16, 1.0);
    draw_text_ex(
        text,
        screen_width() / 2.0 - dims.width / 2.0,
        screen_height() / 2.0 - dims.height / 2.0,
        TextParams {
            font: Default::default(),
            font_size: 50u16,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub struct Statistics {
    passed_intersection: u32,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            passed_intersection: 0,
        }
    }
}

pub enum GameState {
    Menu,
    Game,
    Statistics,
}
