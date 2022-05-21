pub mod dis;
pub mod asm;
pub mod question;

use std::{
	env::temp_dir,
	path::{Path, PathBuf, MAIN_SEPARATOR},
	fs::canonicalize,
};

pub static HEADER: &[u8; 15] = b"MQS-OBJ-V0.0.1\0";
static EXTENSION: &str = "mqso";

pub fn obj_filename(filename: String) -> PathBuf {
	let path = canonicalize(Path::new(&filename)).unwrap();
	let filename = &path.display().to_string()
		.replace(MAIN_SEPARATOR, "%")[1..];

	temp_dir().join(
		Path::new(filename)
			.with_extension(EXTENSION)
	)
}