use std::rc::Rc;

use agent_simulator::prelude::*;

fn make_agent() -> Agent {
    Agent::new(
        "Agent".to_string(),
        Position::default(),
        Box::new(|state| {
            (Action::DoNothing, Cooldown::Forever)
        }),
    )
}

fn main() {
    let agent = make_agent();
    let rock = objects::Rock::new(Position::default());

    let state = State::default()
        .add(Rc::new(agent))
        .add(Rc::new(rock));

    let simulation = SimulationManager::new(state);
}
