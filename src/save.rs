use std::env::current_dir;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug)]
pub enum RustogramSaveError {
	SaveNotFound(std::io::Error),
	CannotCreateFile(std::io::Error),
	CannotOpenFile(std::io::Error),
	CannotWriteFile(std::io::Error),
	TomlParsing(toml::de::Error),
	TomlReadError(toml_edit::TomlError),
}

#[derive(Deserialize)]
pub struct RustogramSave {
	pub id: String,
	pub time: u64,
	pub field: Vec<Vec<u8>>,
}

fn generate_path(id: String) -> PathBuf {
	PathBuf::from(format!(
		"{}/level/{}.nonogram.level",
		current_dir().unwrap().to_string_lossy(),
		id
	))
}

impl RustogramSave {
	pub fn new(id: String, width: usize, height: usize) -> Self {
		Self {
			id: id.to_owned(),
			time: 0,
			field: vec![vec![0; width]; height],
		}
	}

	pub fn load(id: String) -> Result<Self, RustogramSaveError> {
		match toml::from_str(read_to_string(generate_path(id)).unwrap().as_str()) {
			Ok(s) => Ok(s),
			Err(e) => Err(RustogramSaveError::TomlParsing(e)),
		}
	}

	pub fn load_or_create(id: String) -> Result<Self, RustogramSaveError> {
		let path: PathBuf = generate_path(id.to_owned());
		if !path.exists() {
			match File::create(&path) {
				Ok(r) => r,
				Err(e) => return Err(RustogramSaveError::CannotCreateFile(e)),
			};
		}
		self::RustogramSave::load(id)
	}

	pub fn generate_path(id: String) -> PathBuf {
		generate_path(id)
	}

	pub fn path(&self) -> PathBuf {
		generate_path(self.id.to_owned())
	}

	pub fn store(&self) -> Result<(), RustogramSaveError> {
		if let Err(e) = File::open(self.path()) {
			return Err(RustogramSaveError::CannotOpenFile(e));
		}
		todo!("Saving");
		/*match save.write_all(
			format!(
				"id: {}\ntime: {}\nfield: {}",
				self.id, self.time, self.field, self.path
			)
			.as_bytes(),
		) {
			Ok(_) => Ok(()),
			Err(e) => Err(RustogramSaveError::CannotWriteFile(e)),
		}*/
	}
}
