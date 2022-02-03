use bevy::input::system::exit_on_esc_system;
use bevy::log::{Level, LogSettings};
use bevy::prelude::{App, IntoSystem, Msaa, ParallelSystemDescriptorCoercion, WindowDescriptor};
use bevy::window::exit_on_window_close_system;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;

use crate::game::{load_level, pause_on_unfocus, GameState};
use crate::level::Nonogram;
use crate::save::RustogramSave;
use crate::ui::{main_ui, store_window_size, UiState, WindowSizeRes};

mod game;
mod level;
mod save;
mod ui;

fn main() {
	App::new()
		.insert_resource(LogSettings {
			filter: "nonogram=trace".to_string(),
			level: Level::ERROR,
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(EguiPlugin)
		.insert_resource(WindowDescriptor {
			title: "Rustogram".to_string(),
			cursor_visible: true,
			vsync: true,
			resizable: true,
			..Default::default()
		})
		.insert_resource(Msaa { samples: 8 })
		.insert_resource(GameState {
			id: "test".to_string(),
			menu: true,
			running: true,
			game: Nonogram::default(),
			solve: false,
			save: RustogramSave::new("test".to_string(), 2, 2),
			solution: vec![vec![1, 0], vec![1, 1]],
			time: Default::default(),
		})
		.insert_resource(WindowSizeRes::default())
		.init_resource::<UiState>()
		.add_system(store_window_size.system().label("window_size"))
		.add_system(main_ui.system().after("window_size").after("load_level"))
		.add_system(load_level.system().label("load_level"))
		.add_system(exit_on_esc_system.system())
		.add_system(exit_on_window_close_system.system())
		.add_system(pause_on_unfocus.system())
		.run();
}
