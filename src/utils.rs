use std::fs::File;
use std::io::{self, Write};

pub fn write_to_file(file_name: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())
}
