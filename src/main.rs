use macroquad::prelude::*;
use ::rand::thread_rng;
use ::rand::Rng;
use std::default::Default;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Vehicule {
    pub id: i32,
    pub coordonne: Vec2,
    pub vitesse: f32,
    pub direction: Direction,
    pub rotation: f32,
}

/*
Pour gere la taille de la fenaitre
*/
fn window_conf() -> Conf {
    Conf {
        window_title: "IBG Road".to_owned(),
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

impl Vehicule {
    fn new(id: i32, coordonne: Vec2, vitesse: f32, direction: Direction) -> Self {
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
        }
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Up => self.coordonne.y -= self.vitesse,
            Direction::Down => self.coordonne.y += self.vitesse,
            Direction::Left => self.coordonne.x -= self.vitesse,
            Direction::Right => self.coordonne.x += self.vitesse,
        }
    }



    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.rotation = match direction {
            Direction::Up => -90.0,
            Direction::Down => 90.0,
            Direction::Left => 180.0,
            Direction::Right => 0.0,
        };
    }
}

pub fn handle_input(vehicules: &mut Vec<Vehicule>, next_id: &mut i32, rng: &mut impl Rng) {
    let possible_x_values = [500.0, 548.0, 592.0];
    let possible_y_values = [515.0, 560.0, 610.0];
    let direct = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        let random_x = possible_x_values[rng.gen_range(0..possible_x_values.len())];
        let random_y = possible_y_values[rng.gen_range(0..possible_y_values.len())];
        let random_direct = direct[rng.gen_range(0..direct.len())];

    if is_key_pressed(KeyCode::Up) {
        vehicules.push(Vehicule::new(*next_id, vec2(random_x, 990.0), 0.5, Direction::Up));
        *next_id += 1;
    }
    if is_key_pressed(KeyCode::Down) {
        // let random_x = possible_x_values[rng.gen_range(0..possible_x_values.len())];
        vehicules.push(Vehicule::new(*next_id, vec2(random_x - 140.0, 0.0), 0.5, Direction::Down));
        *next_id += 1;
    }
    if is_key_pressed(KeyCode::Left) {
        vehicules.push(Vehicule::new(*next_id, vec2(950.0, random_y - 140.0), 0.5, Direction::Left));
        *next_id += 1;
    }
    if is_key_pressed(KeyCode::Right) {
        // let random_y = possible_y_values[rng.gen_range(0..possible_y_values.len())];
        vehicules.push(Vehicule::new(*next_id, vec2(0.0, random_y), 0.5, Direction::Right));
        *next_id += 1;
    }
    if is_key_pressed(KeyCode::R) {
        match random_direct {
            Direction::Up=> vehicules.push(Vehicule::new(*next_id, vec2(random_x, 990.0), 0.5, Direction::Up)),
            Direction::Down=> vehicules.push(Vehicule::new(*next_id, vec2(random_x - 140.0, 0.0), 0.5, Direction::Down)),
            Direction::Left=> vehicules.push(Vehicule::new(*next_id, vec2(950.0, random_y-140.0), 0.5, Direction::Left)),
            Direction::Right=> vehicules.push(Vehicule::new(*next_id, vec2(0.0 , random_y), 0.5, Direction::Right)),
        }
        *next_id += 1;

        // vehicules.push(Vehicule::new(*next_id, vec2(random_x, )))
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let img: Texture2D = load_texture("./assets/road.png").await.unwrap();
    let car_1: Texture2D = load_texture("./assets/car_blue.png").await.unwrap();
    // let car_2: Texture2D = load_texture("./assets/car_green.png").await.unwrap();
    // let car_3: Texture2D = load_texture("./assets/car_white.png").await.unwrap();
    let mut rng = thread_rng();


    let mut vehicules: Vec<Vehicule> = Vec::new();
    let mut next_id = 1;

    loop {
        handle_input(&mut vehicules, &mut next_id, &mut rng);
        draw_texture(&img, 0.0, 0.0, WHITE);

        for vehicule in &mut vehicules {
            vehicule.update();
            let draw_params = DrawTextureParams {
                rotation: vehicule.rotation.to_radians(),
                ..Default::default()
            };
            draw_texture_ex(
                &car_1,
                vehicule.coordonne.x,
                vehicule.coordonne.y,
                WHITE,
                draw_params,
            );
        }

        next_frame().await;
    }
}