mod config;
mod input_handler;
mod vehicule;

// use crate::config::Direction;
use crate::config::Route;
use crate::input_handler::InputHandler;
use crate::vehicule::Vehicule;
use ::rand::thread_rng;
use macroquad::prelude::*;
use std::time::Duration;

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
    let mut rng = thread_rng();

    let mut vehicules: Vec<Vehicule> = Vec::new();
    let mut next_id = 1;

    let mut input_handler = InputHandler::new(Duration::from_millis(700));

    loop {
        input_handler.handle_input(&mut vehicules, &mut next_id, &mut rng);
        draw_texture(&img, 0.0, 0.0, WHITE);

        let delta_time = get_frame_time();
        for vehicule in &mut vehicules {
            vehicule.update(delta_time);
            let draw_params = DrawTextureParams {
                rotation: vehicule.rotation.to_radians(),
                ..Default::default()
            };

            let car = if vehicule.route == Route::SE
                || vehicule.route == Route::EN
                || vehicule.route == Route::NW
                || vehicule.route == Route::WS
            {
                car_1.clone()
            } else if vehicule.route == Route::NS
                || vehicule.route == Route::SN
                || vehicule.route == Route::WE
                || vehicule.route == Route::EW
            {
                car_2.clone()
            } else {
                car_3.clone()
            };

            draw_texture_ex(
                &car,
                vehicule.coordonne.x,
                vehicule.coordonne.y,
                WHITE,
                draw_params,
            );
        }

        next_frame().await;
    }
}
