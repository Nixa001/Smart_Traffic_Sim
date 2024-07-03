use macroquad::prelude::*;
use std::default::Default;

#[derive(PartialEq)]
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
}

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
        Vehicule {
            id,
            coordonne,
            vitesse,
            direction,
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
}

#[macroquad::main(window_conf)]
async fn main() {
    let img: Texture2D = load_texture("./assets/road.png").await.unwrap();
    let car_1: Texture2D = load_texture("./assets/car_blue.png").await.unwrap();
    let car_2: Texture2D = load_texture("./assets/car_green.png").await.unwrap();
    let car_3: Texture2D = load_texture("./assets/car_white.png").await.unwrap();

    let mut car1 = Vehicule::new(1, vec2(0.0, 510.0), 0.2, Direction::Right);
    let mut car2 = Vehicule::new(1, vec2(0.0, 560.0), 0.2, Direction::Right);
    let mut car3 = Vehicule::new(1, vec2(0.0, 610.0), 0.2, Direction::Right);
    let mut cara = Vehicule::new(1, vec2(1000.0, 370.0), 0.2, Direction::Left);
    let mut carb = Vehicule::new(1, vec2(1000.0, 420.0), 0.2, Direction::Left);
    let mut carc = Vehicule::new(1, vec2(1000.0, 470.0), 0.2, Direction::Left);

    loop {
        draw_texture(&img, 0.0, 0.0, WHITE);

        car1.update();
        car2.update();
        car3.update();
        cara.update();
        carb.update();
        carc.update();
        draw_texture(&car_1, car1.coordonne.x, car1.coordonne.y, WHITE);
        draw_texture(&car_2, car2.coordonne.x, car2.coordonne.y, WHITE);
        draw_texture(&car_3, car3.coordonne.x, car3.coordonne.y, WHITE);
        draw_texture(&car_1, cara.coordonne.x, cara.coordonne.y, WHITE);
        draw_texture(&car_2, carb.coordonne.x, carb.coordonne.y, WHITE);
        draw_texture(&car_3, carc.coordonne.x, carc.coordonne.y, WHITE);

        next_frame().await;
    }
}
