use super::ejected::EjectedData;
use super::request::Request::{self, *};
use super::running::RunningData;
use super::stopped::StoppedData;
use super::typestate::Typestate;

#[test]
pub fn test0() {
  let input_output_pairs: Vec<(Request, Typestate)> = vec![
    (Skip(1), Typestate::Stopped(StoppedData::new(0))),
    (Stop, Typestate::Stopped(StoppedData::new(0))),
    (Run, Typestate::Running(RunningData::new(0))),
    (Skip(2), Typestate::Running(RunningData::new(2))),
    (Skip(-1), Typestate::Running(RunningData::new(1))),
    (Eject, Typestate::Running(RunningData::new(1))),
    (Reset, Typestate::Running(RunningData::new(1))),
    (Run, Typestate::Running(RunningData::new(1))),
    (Stop, Typestate::Stopped(StoppedData::new(1))),
    (Reset, Typestate::Stopped(StoppedData::new(0))),
    (Skip(1), Typestate::Stopped(StoppedData::new(0))),
    (Stop, Typestate::Stopped(StoppedData::new(0))),
    (Run, Typestate::Running(RunningData::new(0))),
    (Skip(1), Typestate::Running(RunningData::new(1))),
    (Stop, Typestate::Stopped(StoppedData::new(1))),
    (Eject, Typestate::Ejected(EjectedData::new(1))),
    (Reset, Typestate::Ejected(EjectedData::new(1))),
    (Run, Typestate::Ejected(EjectedData::new(1))),
    (Skip(2), Typestate::Ejected(EjectedData::new(1))),
    (Stop, Typestate::Ejected(EjectedData::new(1))),
  ];

  let mut type_state = Typestate::default();

  assert_eq!(type_state, Typestate::Stopped(StoppedData::new(0)));

  for (event, expected_type_state) in input_output_pairs {
    type_state = type_state.transit(&event);

    assert_eq!(type_state, expected_type_state);
  }
}
