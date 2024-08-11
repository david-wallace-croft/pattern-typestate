use pattern_typestate::example0::event::Event::EventToB;
use pattern_typestate::example0::typestate::Typestate;

fn main() {
  let typestate_0 = Typestate::default();

  // The transit() method takes ownership of self
  let typestate_1 = typestate_0.transit(&EventToB);

  // Will not compile: Value used after being moved
  // typestate_0.transit(&EventToB);

  let typestate_2: Typestate = match typestate_1 {
    Typestate::StateB(mut state_data_b) => {
      state_data_b.some_mutator_method_unique_to_state_b();

      let _ = state_data_b.some_accessor_method_unique_to_state_b();

      state_data_b.transit_to_state_c()
    },
    _ => typestate_1,
  };

  let _typestate_3 = match typestate_2 {
    Typestate::StateC(mut state_data_c) => {
      // Will not compile; none of these methods exist for state C

      // state_data_c.some_mutator_method_unique_to_state_b();
      //
      // let _ = state_data_c.some_accessor_method_unique_to_state_b();
      //
      // state_data_c.transit_to_state_c()

      state_data_c.some_mutator_method_unique_to_state_c();

      state_data_c.transit_to_state_d()
    },
    _ => typestate_2,
  };
}
