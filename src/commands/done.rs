use crate::storage::Storage;
use anyhow::Result;

pub fn run() -> Result<()> {
    let storage = Storage::default_location()?;

    match storage.load_context()? {
        None => {
            println!("No current context to complete.");
        }
        Some(context) => {
            println!("✅ Task completed: \"{}\"", context.note);
            storage.clear_context()?;
            println!();
            println!("Context cleared. Ready for a new task!");
            println!("Use 'flow note \"...\"' to start tracking a new context.");
        }
    }

    Ok(())
}
