use super::super::event::Event;
use super::ejected::EjectedState;
use super::player_data::PlayerData;
use super::running::RunningState;
use super::stopped::StoppedState;

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Ejected(PlayerData<EjectedState>),
  Running(PlayerData<RunningState>),
  Stopped(PlayerData<StoppedState>),
}

impl Typestate {
  pub fn get_position(&self) -> usize {
    match self {
      Typestate::Ejected(player_data) => player_data.get_position(),
      Typestate::Running(player_data) => player_data.get_position(),
      Typestate::Stopped(player_data) => player_data.get_position(),
    }
  }

  pub fn get_state_name(&self) -> &'static str {
    match self {
      Typestate::Ejected(player_data) => player_data.get_state_name(),
      Typestate::Running(player_data) => player_data.get_state_name(),
      Typestate::Stopped(player_data) => player_data.get_state_name(),
    }
  }

  pub fn transit(
    self,
    event: &Event,
  ) -> Self {
    match self {
      Typestate::Ejected(player_data) => Typestate::Ejected(player_data),
      Typestate::Running(mut player_data) => match event {
        Event::Eject | Event::Reset | Event::Run => {
          Typestate::Running(player_data)
        },
        Event::Skip(delta) => {
          player_data.skip(*delta);

          Typestate::Running(player_data)
        },
        Event::Stop => Typestate::Stopped(player_data.stop()),
      },
      Typestate::Stopped(mut player_data) => match event {
        Event::Eject => Typestate::Ejected(player_data.eject()),
        Event::Reset => {
          player_data.reset();

          Typestate::Stopped(player_data)
        },
        Event::Run => Typestate::Running(player_data.run()),
        Event::Skip(_) | Event::Stop => Typestate::Stopped(player_data),
      },
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    // The default state starts stopped at position zero
    Typestate::Stopped(PlayerData::<StoppedState>::new(0))
  }
}
