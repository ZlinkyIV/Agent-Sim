pub mod simulation {
    pub struct SimulationManager<'a> {
        simulation_state: State<'a>,
    }

    impl<'a> SimulationManager<'a> {
        pub fn new(state: State<'a>) -> Self {
            Self {
                simulation_state: state,
            }
        }

        pub fn step(&mut self) {
            todo!()
            // self.simulation_state = self.simulation_state.next();
        }
    }

    
    #[derive(Default)]
    pub struct State<'a> { 
        objects: Vec<&'a dyn Object>,
    }

    impl<'a> State<'a> {
        // pub fn next(self) -> Self {
        //     todo!()
        // }

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


    #[derive(Clone, Copy, Default)]
    pub struct Position(f32, f32);


    /// An object in the simulation.
    pub trait Object {
        /// The position of the object.
        fn position(&self) -> Position;

        /// Move object. 
        /// 
        /// Immovable objects ignore this call (default behavior).
        fn update_position(&mut self, position: Position) { }

        /// Get next action and update the object's state.
        fn update(&mut self, state: State) -> Action {
            Action::DoNothing
        }
    }


    // pub struct ObjectBehavior {
    //     pub update: &'static dyn FnMut(State) -> Action,
    // }

    // impl Default for ObjectBehavior {
    //     fn default() -> Self {
    //         Self {
    //             update: &|_| Action::default(),
    //         }
    //     }
    // }


    pub enum Action {
        DoNothing,
        // MoveTo(Position),
    }

    impl Default for Action {
        fn default() -> Self {
            Self::DoNothing
        }
    }
}