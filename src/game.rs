use bevy::prelude::{trace, EventReader, ResMut};
use bevy::window::WindowFocused;
use stopwatch::Stopwatch;

use crate::save::RustogramSave;
use crate::Nonogram;

pub struct GameState {
	pub id: String,
	pub running: bool,
	pub menu: bool,
	pub time: Stopwatch,
	pub game: Nonogram,
	pub solve: bool,
	pub save: RustogramSave,
	pub solution: Vec<Vec<i8>>,
}

/// Pause Timer when window looses focus and resume timer when focus gained
pub fn pause_on_unfocus(mut state: ResMut<GameState>, mut win_focused: EventReader<WindowFocused>) {
	if !state.running {
		return;
	};

	for event in win_focused.iter() {
		if event.focused {
			if !state.time.is_running() {
				state.time.start();
			}
			trace!("Window focused -> Timer Resumed");
		} else if !event.focused {
			state.time.stop();
			trace!("Window unfocused -> Timer Paused");
		}
	}
}

pub fn load_level(mut state: ResMut<GameState>) {
	if state.id == state.save.id {
		return;
	}
	trace!("Parsing level file {}", state.save.path().to_string_lossy());
	state.game = Nonogram::from_toml(state.id.to_owned()).unwrap();
	state.time.start();
	state.save.id = state.save.id.to_owned();
}
