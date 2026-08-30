use std::fs;
use std::io;
use std::path::Path;

pub(crate) fn write_text(path: &Path, content: &str) -> io::Result<()> //this is short for Result<(), std::io::Error>
{
    fs::write(path, content)?;
    Ok(())
}

pub(crate) fn read_text(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}
