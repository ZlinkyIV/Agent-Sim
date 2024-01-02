use std::{rc::Rc, cell::RefCell};
use agent_simulator::prelude::*;

fn make_agent() -> Agent {
    Agent::new(
        // "Agent".to_string(),
        Position::default(),
        Box::new(|state| {
            (Action::MoveTo(Position(1.0, 2.0)), Cooldown::Forever)
        }),
    )
}

fn main() {
    let agent = make_agent();
    let rock = prebuilt_entities::Rock::new(Position::default());

    let mut state = State::default();
    state.add(Rc::new(RefCell::new(agent)));
    state.add(Rc::new(RefCell::new(rock)));
    
    println!("Before move:");
    println!("{:?}", state);

    state.next();

    println!("After move:");
    println!("{:?}", state);
}
