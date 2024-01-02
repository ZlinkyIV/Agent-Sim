pub mod prelude {
    pub use crate::state::*;
    pub use crate::entity::*;
}


pub mod state {
    use std::{rc::Rc, cell::RefCell};
    use crate::entity::SimulationEntity;
    
    #[derive(Default, Clone, Debug)]
    pub struct State { 
        entities: Vec<Rc<RefCell<dyn SimulationEntity>>>,
    }

    impl State {
        pub fn next(&mut self) {
            for entity in &self.entities {
                entity.borrow_mut().think(&self);
            }

        }

        pub fn add(&mut self, entity: Rc<RefCell<dyn SimulationEntity>>) {
            self.entities.push(entity);
        }

        /// Add multiple entities to state.
        pub fn add_many<I>(&mut self, entities: I)
        where
            I: IntoIterator<Item = Rc<RefCell<dyn SimulationEntity>>>,
        {
            self.entities.extend(entities);
        }
    }
}


pub mod entity {
    use std::fmt::Debug;

    use crate::state::State;

    /// A position (meters from origin)
    #[derive(Clone, Copy, Default, Debug)]
    pub struct Position(pub f32, pub f32);

    /// Size and weight of an entity
    #[derive(Clone, Copy, Default, Debug)]
    pub struct Dimensions {
        /// Cube size of entity in meters
        pub size: f32,
        /// Weight of entity in kilos
        pub weight: f32,
    }


    pub enum Cooldown {
        Forever,
        Time(u16),
    }


    /// A simulation entity
    pub trait SimulationEntity: Debug {
        /// Position of the entity
        fn position(&self) -> Position;

        /// Dimensions of the entity
        fn dimensions(&self) -> Dimensions;

        /// Now's the time for you to think, Mr. Rock!
        /// Will you do nothing forever like every other boring rock,
        /// or will you actually do something with your life?
        /// 
        /// Returns: Cooldown before next `think` call
        fn think(&mut self, state: &State) -> Cooldown {
            Cooldown::Forever
        }
    }


    /// An action an agent can take
    pub enum Action {
        DoNothing,
        MoveTo(Position),
    }
    impl Default for Action {
        fn default() -> Self {
            Self::DoNothing
        }
    }


    pub struct Agent {
        // name: String,
        position: Position,
        think: Box<dyn FnMut(&State) -> (Action, Cooldown)>,
    }

    impl Agent {
        pub fn new(
            // name: String,
            position: Position,
            think: Box<dyn FnMut(&State) -> (Action, Cooldown)>,
        ) -> Self {
            Self { /* name, */ position, think }
        }
    }

    impl SimulationEntity for Agent {
        fn position(&self) -> Position {
            self.position
        }

        fn dimensions(&self) -> Dimensions {
            Dimensions { size: 1.25, weight: 80.0 }
        }

        fn think(&mut self, state: &State) -> Cooldown {
            let (action, cooldown) = (self.think)(state);

            match action {
                Action::DoNothing => (),
                Action::MoveTo(position) => self.position = position
            }

            cooldown
        }
    }

    // Skip the think function
    impl Debug for Agent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Agent")
                .field("position", &self.position)
                .finish()
        }
    }


    pub mod prebuilt_entities {
        use super::*;

        #[derive(Debug)]
        pub struct Rock {
            position: Position,
        }

        impl Rock {
            pub fn new(position: Position) -> Rock {
                Rock { position }
            }
        }

        impl SimulationEntity for Rock {
            fn position(&self) -> Position {
                self.position
            }

            fn dimensions(&self) -> Dimensions {
                Dimensions { size: 0.75, weight: 1.0 }
            }
        }
    }
}