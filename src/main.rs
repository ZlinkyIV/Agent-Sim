use agent_simulator::simulation::{*, self};

struct Agent { }
impl Object for Agent {
    fn position(&self) -> Position {
        Position::default()
    }

    fn update(&mut self, state: State) -> Action {
        Action::DoNothing
    }
}

fn main() {
    let agent = Agent { };

    let state = State::default()
        .add(&agent);

    let simulation = SimulationManager::new(state);
}
