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
