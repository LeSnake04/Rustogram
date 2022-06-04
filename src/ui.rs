use bevy::prelude::{trace, EventReader, ResMut};
use std::default::Default;

use crate::GameState;
use bevy::window::WindowResized;
use bevy_egui::egui::panel::TopBottomSide::Top;
use bevy_egui::egui::{CentralPanel, Color32, RichText, TopBottomPanel, Ui, Visuals};
use bevy_egui::{egui, EguiContext};
use chrono::Duration as cDuration;

#[derive(Default)]
pub struct UiState {
	pub timer: String,
}

#[derive(Default)]
pub struct WindowSizeRes {
	pub height: f32,
	pub width: f32,
}

pub fn store_window_size(
	mut win_resize: EventReader<WindowResized>,
	mut win_size: ResMut<WindowSizeRes>,
) {
	for event in win_resize.iter() {
		trace!("Window Resized: {},{}", event.height, event.width);
		win_size.height = event.height;
		win_size.width = event.width;
	}
}

pub fn main_ui(
	mut state: ResMut<GameState>,
	mut ectx: ResMut<EguiContext>,
	//win_size: Res<WindowSizeRes>
) {
	// Configure egui
	&ectx.ctx_mut().set_visuals(Visuals::dark());

	fn two_digit_number(number: i64) -> String {
		if number < 10 {
			"0".to_owned() + &number.to_string()
		} else {
			number.to_string()
		}
	}

	/// Function to create egui dialog
	/// ---
	fn egui_dialog(title: &str, content: &str, ctx: &mut EguiContext) {
		egui::Window::new(title.to_string())
			.show(ctx.ctx_mut(), |ui| ui.label(content.to_string()));
	}

	// Status bar
	TopBottomPanel::new(Top, "status_panel")
		//.default_height(win_size.height * 0.1)
		.show(ectx.ctx_mut(), |ui| {
			ui.horizontal(|ui| {
				ui.heading("Rustogram");
				let duration: cDuration = cDuration::from_std(state.time.elapsed()).unwrap();
				ui.label(format!(
					"Time: {}:{}:{}",
					two_digit_number(duration.num_hours()),
					two_digit_number(duration.num_minutes() % 60),
					two_digit_number(duration.num_seconds() % 60)
				));
				if ui
					.button(if !state.solve {
						"Solve Puzzle"
					} else {
						"Puzzle Solved!"
					})
					.clicked()
				{
					state.solve = true
				};
			});
		});

	let game = &state.game;
	let color_values: &Vec<(u8, u8, u8)> = &game.color_values;
	CentralPanel::default().show(ectx.ctx_mut(), |ui| {
		ui.heading(&state.id);
		egui::Grid::new("playing_field").show(ui, |ui| {
			fn draw_color_order(
				ui: &mut Ui,
				color_values: &[(u8, u8, u8)],
				color: &(usize, usize),
			) {
				if color.0 > 0 {
					let color_value = &color_values[(&color.0) - 1];
					ui.label(RichText::new(format!("{}", color.1)).heading().color(
						Color32::from_rgb(color_value.0, color_value.1, color_value.2),
					));
				}
			}

			ui.label("");

			for line in &game.x {
				ui.vertical(|ui| {
					for color in line {
						draw_color_order(ui, color_values, color)
					}
				});
			}

			ui.end_row();
			for row in &game.y {
				ui.horizontal(|ui| {
					for color in row {
						draw_color_order(ui, color_values, color);
					}
				});
				// TODO: Show the actual field
				ui.label(RichText::new("     ").background_color(Color32::from_rgb(0, 0, 0)));
				ui.end_row();
			}
		});
	});
}
