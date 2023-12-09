pub mod prelude {
    pub use crate::state::*;
    pub use crate::object::*;
}


pub mod state {
    use crate::object::Object;

    pub struct SimulationManager<'a> {
        simulation_state: State<'a>,
    }

    impl<'a> SimulationManager<'a> {
        pub fn new(state: State<'a>) -> Self {
            Self {
                simulation_state: state,
            }
        }

        // pub fn step(&mut self) {
        //     todo!()
        // }
    }

    
    #[derive(Default)]
    pub struct State<'a> { 
        objects: Vec<&'a dyn Object>,
    }

    impl<'a> State<'a> {
        /// Add an object to the state.
        pub fn add(mut self, object: &'a impl Object) -> Self {
            self.objects.push(object);
            self
        }

        /// Add multiple objects to state.
        pub fn add_many<I>(mut self, objects: I) -> Self
        where
            I: IntoIterator<Item = &'a dyn Object>,
        {
            self.objects.extend(objects);
            self
        }
    }
}


pub mod object {
    use crate::state::State;


    /// A position (meters from origin)
    #[derive(Clone, Copy, Default)]
    pub struct Position(f32, f32, f32);

    /// Dimensions of an object: size in meters, weight in kilos
    #[derive(Clone, Copy, Default)]
    pub struct Dimensions {
        /// Size of object in meters
        size: f32,
        
        /// Weight of object in kilos
        weight: f32,
    }
    impl Dimensions {
        pub fn new(size: f32, weight: f32) -> Self {
            Self { size, weight }
        }
    }


    /// An action an object can take... I guess. Still working on that idea.
    pub enum Action {
        DoNothing,
        // MoveTo(Position),
    }
    impl Default for Action {
        fn default() -> Self {
            Self::DoNothing
        }
    }


    pub enum Duration {
        Forever,
        Time(u16),
    }


    /// A simulation object
    pub trait Object {
        fn name(&self) -> String;

        /// Position of the object
        fn position(&self) -> Position;

        /// Dimensions of the object
        fn dimensions(&self) -> Dimensions;

        /// Move object.
        /// 
        /// Immovable objects ignore this call (default behavior).
        fn move_position(&mut self, position: &Position) { }

        /// Now's the time for you to think, Mr. Rock!
        /// Will you do nothing forever like every other boring rock,
        /// or will you actually do something with your life?
        fn think(&mut self, state: &State) -> (Action, Duration) {
            (Action::DoNothing, Duration::Forever)
        }
    }


    pub mod objects {
        use super::*;

        pub struct Rock {
            position: Position,
        }
        impl Object for Rock {
            fn name(&self) -> String {
                "Rock".to_string()
            }

            fn position(&self) -> Position {
                self.position
            }

            fn dimensions(&self) -> Dimensions {
                Dimensions::new(0.75, 1.0)
            }
        }
    }
}