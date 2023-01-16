use std::collections::VecDeque;

use crate::model::operation::{LocationStatus, Movement};

#[derive(Debug)]
pub struct Elevator {
    id: u8,
    movement: Movement,
    current_floor: i16,
    work: VecDeque<Movement>,
}

impl Elevator {
    #[must_use]
    pub fn new(id: u8, starting_floor: i16) -> Self {
        Self {
            id,
            movement: Movement::Idle,
            current_floor: starting_floor,
            work: VecDeque::new(),
        }
    }

    #[must_use]
    pub fn location_status(&self) -> LocationStatus {
        let is_idle = self.movement == Movement::Idle;
        LocationStatus::new(self.id, !is_idle, self.current_floor)
    }

    pub fn new_request(&mut self, request: Movement) {
        self.work.push_back(request);
    }

    fn take_request(&mut self, request: Movement) {
        let cf = self.current_floor;
        match request {
            Movement::Down(rf, _) | Movement::Up(rf, _) if cf < rf && cf != rf => {
                self.work.push_front(request);
                self.movement = Movement::Up(cf, rf);
            }
            Movement::Down(rf, _) | Movement::Up(rf, _) if cf > rf && cf != rf => {
                self.work.push_front(request);
                self.movement = Movement::Down(cf, rf);
            }
            _ => self.movement = request,
        };
    }

    pub fn tick(&mut self) {
        let cf = self.current_floor;
        match self.movement {
            Movement::ReturnHome if cf > 0 => {
                tracing::info!("Lift[{}] is RETURNING HOME (Downwards)", self.id);
                self.movement = Movement::Down(cf, 0);
            }
            Movement::ReturnHome if cf < 0 => {
                tracing::info!("Lift[{}] is RETURNING HOME (Upwards)", self.id);
                self.movement = Movement::Up(cf, 0);
            }
            Movement::ReturnHome => {
                tracing::info!("Lift[{}] is opening doors on floor [{}]", self.id, cf);
                self.movement = Movement::OpenDoor;
            }
            Movement::Down(_, until) if cf > until => {
                tracing::info!("Lift[{}] is moving DOWN [{}]", self.id, cf);
                self.current_floor -= 1;
            }
            Movement::Up(_, until) if cf < until => {
                tracing::info!("Lift[{}] is moving UP [{}]", self.id, cf);
                self.current_floor += 1;
            }
            Movement::Up(..) | Movement::Down(..) => {
                tracing::info!("Lift[{}] is opening doors on floor [{}]", self.id, cf);
                self.movement = Movement::OpenDoor;
            }
            Movement::OpenDoor => {
                tracing::info!("Lift[{}] is closing doors on floor [{}]", self.id, cf);
                self.movement = Movement::CloseDoor;
            }
            Movement::CloseDoor => {
                self.movement = Movement::Idle;
            }
            Movement::Idle => {
                if let Some(request) = self.work.pop_front() {
                    self.take_request(request);
                    tracing::info!(
                        "Starting request [{:?}] and [{}] remaining",
                        self.movement,
                        self.work.len()
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::elevator::{Elevator, Movement};

    #[test]
    fn init() {
        let elevator = Elevator::new(1, 0);
        assert_eq!(Movement::Idle, elevator.movement);
        assert_eq!(0, elevator.current_floor);
        assert_eq!(1, elevator.id);
    }

    #[test]
    fn check_tick_down() {
        let mut elevator = Elevator::new(1, 10);
        elevator.movement = Movement::Down(10, 8);
        elevator.tick();
        assert_eq!(9, elevator.current_floor);
        elevator.tick();
        assert_eq!(8, elevator.current_floor);
        let status = elevator.location_status();
        assert_eq!(status.id, 1);
        assert!(status.is_busy);
        assert_eq!(status.floor, 8);
        elevator.tick();
        assert_eq!(8, elevator.current_floor);
    }

    #[test]
    fn check_tick_up() {
        let mut elevator = Elevator::new(1, 0);
        elevator.movement = Movement::Up(0, 3);
        elevator.tick();
        assert_eq!(1, elevator.current_floor);
        elevator.tick();
        assert_eq!(2, elevator.current_floor);
        elevator.tick();
        assert_eq!(3, elevator.current_floor);
        elevator.tick();
        assert_eq!(3, elevator.current_floor);
    }
}
