# Lift With Me - A Modern Elevator system

# Analyse

- Every elevator has its state which contains the current floor, and a set of the next stops.
- The elevators will be moved step by step to their destination, and every step takes a duration depending to the elevator speed.
- A passenger send a Pickup request on the floor “a” to a destination floor “b”.
- The system will search which elevator is the nearest to “a” and is moving at the same direction to the floor “b”.
- The chosen elevator will add in its stops the floor of the pickup request and its destination and continue to move step by step to the pickup request floor.
- If the elevator is at its next stop floor, this stop should be deleted from the elevator state.
- The request should be deleted when the elevator arrive to the requested floor.

In Rust!

# TODO List

- Implement Routes
- Experiment with Tokio Console
- Use [Tokio Console](https://github.com/tokio-rs/console)

