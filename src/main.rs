use agent_simulator::prelude::*;

struct Agent { }
impl Object for Agent {
    fn name(&self) -> String {
        "Agent".to_string()
    }

    fn position(&self) -> Position {
        Position::default()
    }

    fn dimensions(&self) -> Dimensions {
        Dimensions::new(0.1, 1.0)
    }
}

fn main() {
    let agent = Agent { };

    let state = State::default()
        .add(&agent);

    let simulation = SimulationManager::new(state);
}
