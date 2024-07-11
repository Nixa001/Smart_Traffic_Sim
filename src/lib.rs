use macroquad::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

pub const CAR_HEIGHT: f32 = 23.0;
pub const CAR_WIDTH: f32 = 50.0;

const VITESSE_NORMAL: f32 = 1.0;
const VITESSE_MIN: f32 = 0.3;
const VITESSE_RAPID: f32 = 1.5;

const AVANT_INTERSECTION: Vec2 = vec2(300.0, 700.0);
const APRES_INTERSECTION: Vec2 = vec2(400.0, 600.0);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(PartialEq)]
pub enum Turning {
    Left,
    Right,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vehicule {
    pub id: u32,
    pub color: Texture2D,
    pub coordonne: Vec2,
    pub vitesse: (f32, f32),
    pub rotation: f32,
    pub rectangle: (f32, f32),
    pub direction: Direction,
    pub route: Route,
    pub turned: bool,
}

impl Vehicule {
    pub fn new(
        coordonne: Vec2,
        rectangle: (f32, f32),
        color: Texture2D,
        vitesse: (f32, f32),
        id: u32,
        direction: Direction,
        route: Route,
        turned: bool,
    ) -> Vehicule {
        let rotation = match direction {
            Direction::Up => -90.0,
            Direction::Down => 90.0,
            Direction::Left => 180.0,
            Direction::Right => 0.0,
        };
        Vehicule {
            color,
            rectangle,
            coordonne,
            vitesse,
            rotation,
            id,
            direction,
            route,
            turned,
        }
    }

    pub fn drive(&mut self) {
        if self.on_turn_point() && !self.turned {
            self.turn();
        }
        self.coordonne = vec2(
            self.coordonne.x + self.vitesse.0,
            self.coordonne.y + self.vitesse.1,
        );
    }

    pub fn draw(&self, car1: Texture2D, car2: Texture2D, car3: Texture2D) {
        let draw_params = DrawTextureParams {
            dest_size: Some(Vec2::new(CAR_WIDTH, CAR_HEIGHT)),
            rotation: self.rotation.to_radians(),
            ..Default::default()
        };

        let car = match self.route {
            Route::SE | Route::EN | Route::NW | Route::WS => &car1,
            Route::NS | Route::SN | Route::WE | Route::EW => &car2,
            _ => &car3,
        };

        draw_texture_ex(car, self.coordonne.x, self.coordonne.y, WHITE, draw_params);
        // draw_rectangle(
        //     self.coordonne.x,
        //     self.coordonne.y,
        //     CAR_WIDTH,
        //     CAR_HEIGHT,
        //     WHITE,
        // )
        // }
    }

    fn before_cross_road(&self) -> bool {
        match self.direction {
            Direction::Right => self.coordonne.x < AVANT_INTERSECTION.x,
            Direction::Left => self.coordonne.x > AVANT_INTERSECTION.y,
            Direction::Down => self.coordonne.y < AVANT_INTERSECTION.x,
            Direction::Up => self.coordonne.y > AVANT_INTERSECTION.y,
        }
    }

    fn in_stop_zone(&self) -> bool {
        return match self.direction {
            Direction::Right => self.coordonne.x > APRES_INTERSECTION.x - CAR_WIDTH,
            Direction::Left => self.coordonne.x < APRES_INTERSECTION.y,
            Direction::Down => self.coordonne.y > APRES_INTERSECTION.x - CAR_WIDTH,
            Direction::Up => self.coordonne.y < APRES_INTERSECTION.y,
        };
    }

    fn after_cross_road(&self) -> bool {
        match self.direction {
            Direction::Right => self.coordonne.x > APRES_INTERSECTION.y,
            Direction::Left => self.coordonne.x < APRES_INTERSECTION.x,
            Direction::Down => self.coordonne.y > APRES_INTERSECTION.y,
            Direction::Up => self.coordonne.y < APRES_INTERSECTION.x,
        }
    }

    fn on_cross_road(&self) -> bool {
        return !self.before_cross_road() && !self.after_cross_road();
    }

    fn speed_up(&mut self) {
        self.vitesse = match self.direction {
            Direction::Down => (0.0, VITESSE_RAPID),
            Direction::Up => (0.0, -VITESSE_RAPID),
            Direction::Right => (VITESSE_RAPID, 0.0),
            Direction::Left => (-VITESSE_RAPID, 0.0),
        }
    }

    fn is_speed_up(&self) -> bool {
        return self.vitesse.0.abs() == VITESSE_RAPID || self.vitesse.1.abs() == VITESSE_RAPID;
    }

    fn is_slow_down(&self) -> bool {
        return self.vitesse.0.abs() == VITESSE_MIN || self.vitesse.1.abs() == VITESSE_MIN;
    }

    fn slow_down(&mut self) {
        self.vitesse = match self.direction {
            Direction::Down => (0.0, VITESSE_MIN),
            Direction::Up => (0.0, -VITESSE_MIN),
            Direction::Right => (VITESSE_MIN, 0.0),
            Direction::Left => (-VITESSE_MIN, 0.0),
        }
    }

    fn on_turn_point(&self) -> bool {
        return match self.route {
            Route::NW => self.coordonne.y >= 340.0,
            Route::SE => self.coordonne.y <= 625.0,
            Route::WS => self.coordonne.x > 375.0,
            Route::EN => self.coordonne.x < 625.0,

            Route::NE => self.coordonne.y > 515.0,
            Route::SW => self.coordonne.y < 485.0,
            Route::WN => self.coordonne.x > 515.0,
            Route::ES => self.coordonne.x < 485.0,
            _ => false,
        };
    }

    fn turn(&mut self) {
        let vitesse = self.vitesse;
        let r = self.rectangle;
        self.rectangle.0 = r.1;
        self.rectangle.1 = r.0;
        self.turned = true;

        match self.route {
            Route::NE => {
                self.vitesse.0 = vitesse.1;
                self.vitesse.1 = vitesse.0;
                self.direction = Direction::Right;
                self.coordonne.y = 515.0;
                self.rotation = 0.0;
            }
            Route::SW => {
                self.vitesse.0 = vitesse.1;
                self.vitesse.1 = vitesse.0;
                self.direction = Direction::Left;
                self.coordonne.y = 475.0;
                self.rotation = 180.0;
            }
            Route::WN => {
                self.vitesse.0 = -vitesse.1;
                self.vitesse.1 = -vitesse.0;
                self.direction = Direction::Up;
                self.coordonne.x = 500.0;
                self.rotation = 270.0;
            }
            Route::ES => {
                self.vitesse.0 = -vitesse.1;
                self.vitesse.1 = -vitesse.0;
                self.direction = Direction::Down;
                self.coordonne.x = 455.0;
                self.rotation = 90.0;
            }
            Route::NW => {
                self.vitesse.0 = -vitesse.1;
                self.vitesse.1 = -vitesse.0;
                self.direction = Direction::Left;
                self.coordonne.y = 370.0;
                self.rotation = 180.0;
            }
            Route::SE => {
                self.vitesse.0 = -vitesse.1;
                self.vitesse.1 = -vitesse.0;
                self.coordonne.y = 610.0;
                self.direction = Direction::Right;
                self.rotation = 0.0;
            }
            Route::WS => {
                self.direction = Direction::Down;
                self.vitesse.0 = vitesse.1;
                self.vitesse.1 = vitesse.0;
                self.coordonne.x = 360.0;
                self.rotation = 90.0;
            }
            Route::EN => {
                self.direction = Direction::Up;
                self.vitesse.0 = vitesse.1;
                self.vitesse.1 = vitesse.0;
                self.coordonne.x = 600.0;
                self.rotation = 270.0;
            }
            _ => return,
        }
    }

    fn drive_away(&self) -> bool {
        return match self.direction {
            Direction::Right => self.coordonne.x > 1000.0,
            Direction::Left => self.coordonne.x < 0.0 - CAR_WIDTH,
            Direction::Down => self.coordonne.y > 1000.0,
            Direction::Up => self.coordonne.y < 0.0 - CAR_WIDTH,
        };
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Route {
    NS,
    SN,
    WE,
    EW,

    NW,
    SE,
    WS,
    EN,

    NE,
    SW,
    WN,
    ES,
}

impl Route {
    fn get_coordinates(&self) -> Vec2 {
        match *self {
            Route::NS => vec2(410.0, 0.0),
            Route::SN => vec2(550.0, 1000.0),
            Route::WE => vec2(0.0, 560.0),
            Route::EW => vec2(1000.0, 420.0),

            Route::NW => vec2(360.0, 0.0),
            Route::SE => vec2(600.0, 1000.0),
            Route::WS => vec2(0.0, 605.0),
            Route::EN => vec2(1000.0, 365.0),

            Route::NE => vec2(460.0, 0.0),
            Route::SW => vec2(500.0, 1000.0),
            Route::WN => vec2(0.0, 515.0),
            Route::ES => vec2(1000.0, 465.0),
        }
    }
    fn get_speed(&self) -> (f32, f32) {
        match *self {
            Route::NS => (0.0, VITESSE_NORMAL),
            Route::NW => (0.0, VITESSE_NORMAL),
            Route::NE => (0.0, VITESSE_NORMAL),

            Route::SN => (0.0, -VITESSE_NORMAL),
            Route::SE => (0.0, -VITESSE_NORMAL),
            Route::SW => (0.0, -VITESSE_NORMAL),

            Route::WE => (VITESSE_NORMAL, 0.0),
            Route::WS => (VITESSE_NORMAL, 0.0),
            Route::WN => (VITESSE_NORMAL, 0.0),

            Route::EW => (-VITESSE_NORMAL, 0.0),
            Route::EN => (-VITESSE_NORMAL, 0.0),
            Route::ES => (-VITESSE_NORMAL, 0.0),
        }
    }
    fn get_direction(&self) -> Direction {
        match *self {
            Route::NS => Direction::Down,
            Route::NW => Direction::Down,
            Route::NE => Direction::Down,

            Route::SN => Direction::Up,
            Route::SE => Direction::Up,
            Route::SW => Direction::Up,

            Route::WE => Direction::Right,
            Route::WS => Direction::Right,
            Route::WN => Direction::Right,

            Route::EW => Direction::Left,
            Route::EN => Direction::Left,
            Route::ES => Direction::Left,
        }
    }

    fn not_allowed_to_go(&self) -> Vec<Route> {
        match *self {
            Route::NS => vec![Route::EW, Route::WE, Route::WN, Route::SW],
            Route::SN => vec![Route::NE, Route::WE, Route::ES, Route::EW],
            Route::WE => vec![Route::NS, Route::SW, Route::SN, Route::ES],
            Route::EW => vec![Route::NS, Route::NE, Route::SN, Route::WN],

            Route::NW => vec![],
            Route::SE => vec![],
            Route::WS => vec![],
            Route::EN => vec![],

            Route::NE => vec![Route::EW, Route::SN, Route::SW, Route::WN, Route::ES],
            Route::SW => vec![Route::NS, Route::NE, Route::WE, Route::WN, Route::ES],
            Route::WN => vec![Route::NS, Route::NE, Route::SW, Route::EW, Route::ES],
            Route::ES => vec![Route::NE, Route::SN, Route::SW, Route::WE, Route::WN],
        }
    }
}

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
            min_velocity: f32::MAX,
            close_calls: 0,
            collapse: 0,
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
                                res = res && !cars.get(f).unwrap().is_speed_up();
                            });
                            res
                        }
                        None => true,
                    };
                    can_go = can_go && (self.occupied_tracks.get(r).is_none() || not_speed_up);
                });

                if !cars_on_cross_road.is_none() {
                    let mut all_cars = cars_on_cross_road.unwrap().clone();
                    if !car.before_cross_road() && !car.is_speed_up() {
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
                    } else if car.after_cross_road() {
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
                if car.before_cross_road() && ind >= 1 {
                    if !cars.get(&cars_ids[ind - 1]).is_none()
                        && cars.get(&cars_ids[ind - 1]).unwrap().is_slow_down()
                    {
                        car.slow_down();
                    } else {
                        car.vitesse = route.get_speed();
                    }
                }
                let mut car_clone = car.clone();
                car_clone.drive();
                if (car_clone.is_speed_up() || !car_clone.in_stop_zone())
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
                    car.drive();
                    if car.drive_away() {
                        self.cars.remove(car_id);
                        self.number_of_passed_vehicles += 1;
                    }
                }
            }
            for car in self.cars.values() {
                let speed = (car.vitesse.0.powi(2) + car.vitesse.1.powi(2)).sqrt();
                self.max_velocity = self.max_velocity.max(speed);
                self.min_velocity = self.min_velocity.min(speed);
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
