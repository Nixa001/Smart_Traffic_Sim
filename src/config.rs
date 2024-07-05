pub const VITESSE_MIN: f32 = 0.1;
pub const VITESSE_NORMAL: f32 = 0.5;
pub const VITESSE_MAX: f32 = 1.8;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Route {
    NE,
    NS,
    NW,
    SN,
    SE,
    SW,
    EN,
    EW,
    ES,
    WE,
    WS,
    WN,
}
