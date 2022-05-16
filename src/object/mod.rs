pub mod dis;
pub mod gen;
pub mod question;

use std::{env::temp_dir, path::{Path, PathBuf, MAIN_SEPARATOR}};

pub static HEADER: &[u8; 15] = b"MQS-OBJ-V0.0.1\0";
static EXTENSION: &str = "mqso";

pub fn obj_filename(filename: String) -> PathBuf {
	let filename = filename.replace(MAIN_SEPARATOR, "%");

	temp_dir().join(
		Path::new(&filename)
			.with_extension(EXTENSION)
	)
}