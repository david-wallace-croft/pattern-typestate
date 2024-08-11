use pattern_typestate::example0::event::Event::EventToB;
use pattern_typestate::example0::typestate::Typestate;

fn main() {
  let original_typestate = Typestate::default();

  // The transit() method takes ownership of self
  let new_typestate = original_typestate.transit(&EventToB);

  // Does not compile: Value used after being moved
  // original_type_state.transit(&Event0);

  // if let TypeState::StateB(state_data_b) = &new_type_state {
  //   let _ = state_data_b.some_accessor_method_unique_to_state_b();
  // }

  let _new_new_typestate: Typestate = match new_typestate {
    Typestate::StateB(mut state_data_b) => {
      state_data_b.some_mutator_method_unique_to_state_b();

      let _ = state_data_b.some_accessor_method_unique_to_state_b();

      state_data_b.transit_to_state_c()
    },
    _ => new_typestate,
  };
}
