use std::collections::VecDeque;

use crate::model::operation::{Command, LocationStatus, Movement};
use tokio::sync::broadcast::{Receiver, Sender};

/// Creates a new Elevator
pub async fn add_lift(id: u8, mut cmd_rx: Receiver<Command>, status_tx: Sender<LocationStatus>) {
    let mut lift = Elevator::new(id, 10);
    lift.add_request(Movement::ReturnHome);
    tracing::info!("Creating new Lift with Id=[{}]", id);

    while let Ok(cmd) = cmd_rx.recv().await {
        match cmd {
            Command::Tick => lift.tick(),
            Command::LocationStatus => {
                let current_status = lift.location_status();
                if let Err(e) = status_tx.send(current_status) {
                    tracing::error!("Unable to send status due to [{e}]");
                }
            }
            Command::Lift(lift_id, request) => {
                if lift_id == id {
                    lift.add_request(request);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Elevator {
    id: u8,
    movement: Movement,
    current_floor: i16,
    work: VecDeque<Movement>,
}

impl Elevator {
    #[must_use]
    /// Initalisation method to create a new Elevator
    pub fn new(id: u8, starting_floor: i16) -> Self {
        Self {
            id,
            movement: Movement::Idle,
            current_floor: starting_floor,
            work: VecDeque::new(),
        }
    }

    #[must_use]
    /// Obtain the current status of the Lift
    pub fn location_status(&self) -> LocationStatus {
        let is_idle = self.movement == Movement::Idle;
        LocationStatus::new(self.id, !is_idle, self.current_floor)
    }

    /// Add a new Movement request to the Elevator work queue
    pub fn add_request(&mut self, request: Movement) {
        if request != Movement::Idle {
            self.work.push_back(request);
        }
    }

    /// Tick through time
    pub fn tick(&mut self) {
        match self.movement {
            Movement::ReturnHome if self.current_floor > 0 => {
                tracing::info!("Lift[{}] is RETURNING HOME (Downwards)", self.id);
                self.movement = Movement::Down(self.current_floor, 0);
            }
            Movement::ReturnHome if self.current_floor < 0 => {
                tracing::info!("Lift[{}] is RETURNING HOME (Upwards)", self.id);
                self.movement = Movement::Up(self.current_floor, 0);
            }
            Movement::ReturnHome => {
                self.movement = Movement::OpenDoor;
            }
            Movement::Down(_, until) if self.current_floor > until => {
                tracing::info!("Lift[{}] is moving DOWN", self.id);
                self.current_floor -= 1;
            }
            Movement::Up(_, until) if self.current_floor < until => {
                tracing::info!("Lift[{}] is moving UP", self.id);
                self.current_floor += 1;
            }
            Movement::Up(..) | Movement::Down(..) => {
                tracing::info!("Lift[{}] is opening doors", self.id);
                self.movement = Movement::OpenDoor;
            }
            Movement::OpenDoor => {
                tracing::info!("Lift[{}] is closing doors", self.id);
                self.movement = Movement::CloseDoor;
            }
            Movement::CloseDoor => {
                self.movement = Movement::Idle;
            }
            Movement::Idle => {
                if let Some(request) = self.work.pop_front() {
                    tracing::info!("Lift[{}] is starting request [{request:?}]", self.id);
                    self.movement = request;
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
