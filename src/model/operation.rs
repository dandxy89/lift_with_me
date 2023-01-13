#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Tick,
    LocationStatus,
    Lift(u8, Movement),
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

#[derive(Clone, Debug, PartialEq, Eq)]
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
