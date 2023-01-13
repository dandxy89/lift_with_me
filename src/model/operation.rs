use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Tick,
    RequestLocation,
    SendLocation(LocationStatus),
    Lift(u8, Movement),
    Register(LocationStatus),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Movement {
    // Lift is moving from Floor (left) to (right)
    Down(i16, i16),
    Up(i16, i16),
    // Lift is Idle and ready for work
    Idle,
    // Lift is returning to floor 0
    ReturnHome,
    // Lift is letting passengers out
    OpenDoor,
    CloseDoor,
}

impl Movement {
    #[must_use]
    pub fn get_direction(to_floor: i16, from_floor: i16) -> Option<Self> {
        match to_floor - from_floor {
            v if v < 0 => Some(Movement::Down(from_floor, to_floor)),
            v if v > 0 => Some(Movement::Up(from_floor, to_floor)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LocationStatus {
    pub id: u8,
    pub is_busy: bool,
    pub floor: i16,
}

impl LocationStatus {
    #[must_use]
    pub fn new(id: u8, is_busy: bool, current_floor: i16) -> Self {
        Self {
            id,
            is_busy,
            floor: current_floor,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Passenger {
    pub from_floor: i16,
    pub to_floor: i16,
}
