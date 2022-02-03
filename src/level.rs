use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;

use serde_derive::Deserialize;

/// Nonogram
/// ---
///
/// Describes the Structure of a Nonogram
#[derive(Deserialize, Clone)]
pub struct Nonogram {
	pub id: String,
	/// Height of the Nonogram
	/// ---
	pub height: u8,
	/// Width of the Nonogram
	/// ---
	pub width: u8,
	pub color_names: Vec<String>,
	pub color_values: Vec<(u8, u8, u8)>,
	pub x: Vec<Vec<(usize, usize)>>,
	/// Colors on the y axis
	/// ---
	/// Format:
	/// ```
	/// [[[<Color id>, <number of tiles>],[<Color id>, <number of tiles>]]]
	/// ```
	pub y: Vec<Vec<(usize, usize)>>,
	pub path: PathBuf,
}

#[derive(Debug)]
pub enum NonogramError {
	TomlParsing(toml::de::Error),
	//NonogramParsing(String),
}

fn generate_path(id: String) -> PathBuf {
	PathBuf::from(format!(
		"{}/level/{}.nonogram.level",
		current_dir().unwrap().to_string_lossy(),
		id
	))
}

impl Nonogram {
	pub fn default() -> Self {
		Self {
			id: "test".to_string(),
			height: 2,
			width: 2,
			color_names: vec!["White".to_string()],
			color_values: vec![(255, 255, 255)],
			x: vec![vec![(1, 1), (0, 1)]],
			y: vec![vec![(1, 2)]],
			path: generate_path("test".to_string()),
		}
	}

	pub fn generate_path(id: String) -> PathBuf {
		generate_path(id)
	}

	pub fn path(&self) -> PathBuf {
		generate_path(self.id.to_owned())
	}

	pub fn from_toml(id: String) -> Result<Nonogram, NonogramError> {
		match toml::from_str(read_to_string(generate_path(id)).unwrap().as_str()) {
			Ok(r) => Ok(r),
			Err(e) => Err(NonogramError::TomlParsing(e)),
		}
	}
}
