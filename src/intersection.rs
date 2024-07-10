use macroquad::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::{Route, Vehicule, constants::*};

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    tracks: HashMap<Route, Vec<u32>>,
    car_id: u32,
    occupied_tracks: HashMap<Route, HashSet<u32>>,
    cars: HashMap<u32, Vehicule>,
    queue: VecDeque<u32>,

    pub number_of_passed_vehicles: u32,
    pub max_velocity: f32,
    pub min_velocity: f32,
    pub collapse: u32,
    pub close_calls: u32,
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            tracks: HashMap::new(),
            car_id: 0,
            occupied_tracks: HashMap::new(),
            cars: HashMap::new(),
            queue: VecDeque::new(),
            number_of_passed_vehicles: 0,
            max_velocity: 0.0,
            min_velocity: 0.0,
            collapse: 0,
            close_calls: 0,
        }
    }

    pub fn add_car(&mut self, routes: Vec<Route>, color: Vec<Texture2D>) {
        let route: Route = generate_route(routes);
        if !self.can_add(route) {
            return;
        }

        self.car_id += 1;

        let mut rectangle: (f32, f32) = (CAR_WIDTH, CAR_HEIGHT);
        let direction = route.get_direction();
        if direction == Direction::Up || direction == Direction::Down {
            rectangle = (CAR_HEIGHT, CAR_WIDTH);
        }
        let car_color = match route {
            Route::SE | Route::EN | Route::NW | Route::WS => color[0].clone(),
            Route::NS | Route::SN | Route::WE | Route::EW => color[1].clone(),
            _ => color[2].clone(),
        };

        let car = Vehicule::new(
            route.get_coordinates(),
            rectangle,
            car_color,
            route.get_speed(),
            self.car_id,
            direction,
            route,
            false,
        );

        let current_cars_on_track = self.tracks.get_mut(&route);
        let mut cars = match current_cars_on_track {
            Some(value) => value.to_vec(),
            None => Vec::new(),
        };
        cars.push(car.id);
        self.tracks.get_mut(&route);
        self.tracks.insert(route, cars.clone());
        self.cars.insert(car.id, car);
    }

    fn can_add(&mut self, route: Route) -> bool {
        let start_coordinates = route.get_coordinates();
        let cars = self.tracks.get_mut(&route);
        return match cars {
            Some(cars) => {
                let last_car_id = cars.as_slice().last().unwrap();
                let last_car_position = self.cars.get(last_car_id).unwrap().coordonne;
                if (route == Route::NS || route == Route::NW || route == Route::NE)
                    && last_car_position.y <= start_coordinates.y + CAR_WIDTH * 2.0
                {
                    return false;
                }
                if (route == Route::SN || route == Route::SE || route == Route::SW)
                    && last_car_position.y + CAR_WIDTH * 2.0 >= start_coordinates.y
                {
                    return false;
                }
                if (route == Route::WE || route == Route::WS || route == Route::WN)
                    && last_car_position.x <= start_coordinates.x + CAR_WIDTH * 2.0
                {
                    return false;
                }
                if (route == Route::EW || route == Route::EN || route == Route::ES)
                    && last_car_position.x + CAR_WIDTH * 2.0 >= start_coordinates.x
                {
                    return false;
                }
                true
            }
            None => true,
        };
    }

    pub fn draw_cars(&self, car1: Texture2D, car2: Texture2D, car3: Texture2D) {
        for (_route, cars) in self.tracks.iter() {
            cars.iter().for_each(|id| {
                let car = self.cars.get(id).unwrap();
                car.draw(car1.clone(), car2.clone(), car3.clone());
            })
        }
    }

    pub fn drive_cars(&mut self) {
        // Implementation of drive_cars method
        // ... (copy the existing implementation here)
    }

    pub fn remove_cars(&mut self) {
        let mut map: HashMap<Route, Vec<u32>> = HashMap::new();
        for (route, cars) in self.tracks.iter() {
            let mut left_cars: Vec<u32> = vec![];
            cars.iter().for_each(|c| {
                if self.cars.contains_key(c) {
                    left_cars.push(*c);
                }
            });
            if left_cars.len() > 0 {
                map.insert(*route, left_cars);
            }
        }
        self.tracks = map;
    }
}

fn generate_route(routes: Vec<Route>) -> Route {
    let n: usize = rand::gen_range(0, routes.len());
    return routes[n];
}

fn intersect(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    // Implementation of intersect function
    // ... (copy the existing implementation here)
}