use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{self, prelude::*};

const EOF: &str = "CTRL+D";

fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Expects a file to write the notes to. Exiting.");
        std::process::exit(1);
    });
    let c = get_note_from_terminal()?;
    write_to_file(c, path)?;
    Ok(())
}

fn get_note_from_terminal() -> Result<String> {
    println!("Add your note here.\nPress {} when finished.", EOF);
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn write_to_file(c: String, path: String) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;
    write!(&file, "{}", c)?;
    Ok(())
}
