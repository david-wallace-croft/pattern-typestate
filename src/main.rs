use pattern_typestate::example0::event::Event::EventToB;
use pattern_typestate::example0::type_state::TypeState;

fn main() {
  let original_type_state = TypeState::default();

  // The transit() method takes ownership of self
  let new_type_state = original_type_state.transit(&EventToB);

  // Does not compile: Value used after being moved
  // original_type_state.transit(&Event0);

  if let TypeState::StateB(mut state_data_b) = new_type_state {
    state_data_b.some_function_unique_to_state_b();
  }

  // new_type_state.transit(&Event0);
}
