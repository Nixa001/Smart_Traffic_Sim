use crate::config::Route;
use crate::input_handler::InputHandler;
use crate::vehicule::Vehicule;
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

        // Update vehicles and check for collisions
        for i in 0..vehicules.len() {
            let mut current_vehicle = vehicules[i].clone();
            current_vehicle.update(delta_time);

            for j in 0..vehicules.len() {
                if i != j
                    && vehicules[i].clone().route != Route::SE
                    && vehicules[i].clone().route != Route::EN
                    && vehicules[i].clone().route != Route::NW
                    && vehicules[i].clone().route != Route::WS
                    && vehicules[j].clone().route != Route::SE
                    && vehicules[j].clone().route != Route::EN
                    && vehicules[j].clone().route != Route::NW
                    && vehicules[j].clone().route != Route::WS
                {
                    let mut other_vehicle = vehicules[j].clone();
                    if current_vehicle.detect_collision_large(&other_vehicle) {
                        current_vehicle.adjust_speed_based_on_sensors(&mut other_vehicle);
                    }
                    vehicules[j] = other_vehicle;
                }
            }

            vehicules[i] = current_vehicle;
        }

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
        next_frame().await;
    }
}
