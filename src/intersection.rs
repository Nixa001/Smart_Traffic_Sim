use crate::constants::*;
use crate::route::*;
use crate::vehicule::*;
use macroquad::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

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
        for (route, cars_ids) in self.tracks.iter() {
            for (ind, car_id) in cars_ids.iter().enumerate() {
                let cars = self.cars.clone();
                let cars_on_cross_road = self.occupied_tracks.get(route);
                let car: &mut Vehicule = self.cars.get_mut(car_id).unwrap();
                let mut can_go = route.not_allowed_to_go().len() == 0
                    || self.queue.is_empty()
                    || self.queue[0] == car.id;

                route.not_allowed_to_go().iter().for_each(|r| {
                    let not_speed_up = match self.occupied_tracks.get(r) {
                        Some(a) => {
                            let mut res = true;
                            a.iter().for_each(|f| {
                                res = res && !cars.get(f).unwrap().is_vitesse_max();
                            });
                            res
                        }
                        None => true,
                    };
                    can_go = can_go && (self.occupied_tracks.get(r).is_none() || not_speed_up);
                });

                if !cars_on_cross_road.is_none() {
                    let mut all_cars = cars_on_cross_road.unwrap().clone();
                    if !car.avant_intersect() && !car.is_vitesse_max() {
                        if can_go {
                            car.speed_up();
                            if !self.queue.is_empty() && self.queue[0] == car.id {
                                self.queue.pop_front();
                            }
                        } else {
                            car.slow_down();
                            if !self.queue.contains(&car.id) {
                                self.queue.push_back(car.id);
                            }
                        }
                        all_cars.insert(car.id);
                    } else if car.after_intersect() {
                        all_cars.remove(&car.id);
                    }
                    if all_cars.is_empty() {
                        self.occupied_tracks.remove(route);
                    } else {
                        self.occupied_tracks.insert(*route, all_cars);
                    }
                } else if car.on_cross_road() {
                    let cars = HashSet::from_iter(vec![car.id]);
                    if can_go {
                        car.speed_up();
                        if !self.queue.is_empty() && self.queue[0] == car.id {
                            self.queue.pop_front();
                        }
                    } else {
                        car.slow_down();
                        if !self.queue.contains(&car.id) {
                            self.queue.push_back(car.id);
                        }
                    }
                    self.occupied_tracks.insert(*route, cars);
                }
                if car.avant_intersect() && ind >= 1 {
                    if !cars.get(&cars_ids[ind - 1]).is_none()
                        && cars.get(&cars_ids[ind - 1]).unwrap().is_vitesse_min()
                    {
                        car.slow_down();
                    } else {
                        car.vitesse = route.get_speed();
                    }
                }
                let mut car_clone = car.clone();
                car_clone.move_car();
                if (car_clone.is_vitesse_max() || !car_clone.in_stop_zone())
                    && !cars.values().any(|c| {
                        c.id != car_clone.id
                            && intersect(
                                car_clone.coordonne,
                                c.coordonne,
                                vec2(
                                    car_clone.coordonne.x + car_clone.rectangle.0 + 5.0,
                                    car_clone.coordonne.y + car_clone.rectangle.1 + 5.0,
                                ),
                                vec2(
                                    c.coordonne.x + c.rectangle.0 + 5.0,
                                    c.coordonne.y + c.rectangle.1 + 5.0,
                                ),
                            )
                    })
                {
                    car.move_car();
                    if car.is_out_of_road() {
                        self.cars.remove(car_id);
                        self.number_of_passed_vehicles += 1;
                    }
                }
            }
        }
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
    return ((((a.x >= b.x && a.x <= d.x) || (c.x >= b.x && c.x <= d.x))
        && ((a.y >= b.y && a.y <= d.y) || (c.y >= b.y && c.y <= d.y)))
        || (((b.x >= a.x && b.x <= c.x) || (d.x >= a.x && d.x <= c.x))
            && ((b.y >= a.y && b.y <= c.y) || (d.y >= a.y && d.y <= c.y))))
        || ((((a.x >= b.x && a.x <= d.x) || (c.x >= b.x && c.x <= d.x))
            && ((b.y >= a.y && b.y <= c.y) || (d.y >= a.y && d.y <= c.y)))
            || (((b.x >= a.x && b.x <= c.x) || (d.x >= a.x && d.x <= c.x))
                && ((a.y >= b.y && a.y <= d.y) || (c.y >= b.y && c.y <= d.y))));
}
