use std::env::current_dir;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

use serde::Deserialize;

fn base_path() -> String {
	current_dir().unwrap().to_string_lossy().parse().unwrap()
}

#[derive(Debug)]
pub enum Error {
	SaveNotFound(std::io::Error),
	CannotCreateFile(std::io::Error),
	CannotOpenFile(std::io::Error),
	CannotWriteFile(std::io::Error),
	TomlParsing(toml::de::Error),
	TomlWriteError(toml_edit::TomlError),
	UnknownError(String),
	NotImplemented(String),
}

#[derive(Deserialize)]
pub struct RustogramSave {
	pub id: String,
	pub time: u64,
	pub field: Vec<Vec<u8>>,
}

impl RustogramSave {
	pub fn new(id: String, width: usize, height: usize) -> Self {
		Self {
			id: id.to_owned(),
			time: 0,
			field: vec![vec![0; width]; height],
		}
	}

	pub fn load(id: String) -> Result<Self, Error> {
		match toml::from_str(read_to_string(Self::generate_path(id)).unwrap().as_str()) {
			Ok(s) => Ok(s),
			Err(e) => Err(Error::TomlParsing(e)),
		}
	}

	pub fn load_or_create(id: String) -> Result<Self, Error> {
		let path: PathBuf = Self::generate_path(id.to_owned());
		if !path.exists() {
			if let Err(e) = File::create(&path) {
				return Err(Error::CannotCreateFile(e));
			}
		}
		Self::load(id)
	}

	pub fn generate_path(id: String) -> PathBuf {
		PathBuf::from(format!("{}/level/{}.nonogram.level", base_path(), id))
	}

	pub fn path(&self) -> PathBuf {
		Self::generate_path(self.id.to_owned())
	}

	fn to_string(&self) -> Result<String, Error> {
		let mut field: String = "".into();
		for x in &self.field {
			let mut field_row: String = "".into();

			for y in x {
				let last: &u8 = match x.last() {
					Some(r) => r,
					None => {
						return Err(Error::UnknownError(format!(
							"Error while parsing Field in row {}: Last object invalid",
							&self.field.iter().position(|r| r == x).unwrap()
						)))
					}
				};
				if y == last {
					field_row.push_str(&*format!("{}", y));
				} else {
					field_row.push_str(&*format!("{}, ", y));
				};
			}

			field.push_str(&field_row);
		}
		Ok(format!(
			"id: {} \ntime: {} \nfield: {}",
			self.id, self.time, field
		))
	}

	fn store(&self) -> Result<(), Error> {
		let mut save = match File::open(self.path()) {
			Ok(f) => f,
			Err(e) => return Err(Error::CannotOpenFile(e)),
		};
		return Err(Error::NotImplemented("Saving of File".to_string()));
		let content: String = self.to_string()?;
		return match save.write_all(format!("{}", content).as_bytes()) {
			Ok(_) => Ok(()),
			Err(e) => Err(Error::CannotWriteFile(e)),
		};
	}
}
