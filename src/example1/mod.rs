use self::event::Event::Initialize;
use self::typestate::Typestate;

pub mod event;
pub mod initialized;
pub mod running;
pub mod state_data_e;
pub mod suspended;
pub mod typestate;
pub mod uninitialized;

#[cfg(test)]
mod test;

pub fn example1() {
  let typestate_0 = Typestate::default();

  // The transit() method takes ownership of self
  let typestate_1 = typestate_0.transit(&Initialize);

  // Will not compile: Value used after being moved
  // typestate_0.transit(&EventToB);

  let typestate_2: Typestate = match typestate_1 {
    Typestate::Initialized(mut state_data_b) => {
      state_data_b.some_mutator_method_unique_to_state_b();

      let _ = state_data_b.some_accessor_method_unique_to_state_b();

      state_data_b.start()
    },
    _ => typestate_1,
  };

  let _typestate_3 = match typestate_2 {
    Typestate::Running(mut state_data_c) => {
      // Will not compile; none of these methods exist for state C

      // state_data_c.some_mutator_method_unique_to_state_b();
      //
      // let _ = state_data_c.some_accessor_method_unique_to_state_b();
      //
      // state_data_c.transit_to_state_c()

      state_data_c.some_mutator_method_unique_to_state_c();

      state_data_c.stop()
    },
    _ => typestate_2,
  };
}
