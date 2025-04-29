use anyhow::Result;
use std::{fs::File, io::Write};

use crate::{cli::New, note::Note};

pub fn handle_add(params: New) -> Result<()> {
    let note = Note::new(params.title.as_str(), "");

    // TODO: if error message are not good, do the full if let / else with better error message
    let mut file = File::create_new(&note.path)?;
    println!("Creating new note: {}", note.path.display());
    file.write_all(
        note.to_string()
            .expect("Failed to serialize note to string")
            .as_bytes(),
    )?;

    // TODO: add the option to tag a note directly at the creation
    // TODO: add an option to be able to open the editor and edit the note directly

    Ok(())
}
