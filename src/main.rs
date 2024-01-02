use std::rc::Rc;
use agent_simulator::prelude::*;

fn make_agent() -> Agent {
    Agent::new(
        // "Agent".to_string(),
        Position::default(),
        Box::new(|state| {
            (Action::DoNothing, Cooldown::Forever)
        }),
    )
}

fn main() {
    let agent = make_agent();
    let rock = prebuilt_entities::Rock::new(Position::default());

    let state = {
        let mut new_state = State::default();
        new_state.add(Rc::new(agent));
        new_state.add(Rc::new(rock));
        new_state
    };
    
    println!("{:?}", state);
}
