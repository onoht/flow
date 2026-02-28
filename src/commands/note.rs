use crate::context::Context;
use crate::git;
use crate::storage::Storage;
use anyhow::Result;
use std::env;

pub fn run(note: String) -> Result<()> {
    let storage = Storage::default_location()?;

    // Try to detect git information
    let current_dir = env::current_dir()?;
    let git_info = git::detect_git_info(&current_dir)?;

    // Create context with or without git info
    let context = match git_info {
        Some(info) => {
            println!("📍 Detected: {} ({})", info.repo_name, info.branch);
            Context::new_with_git(note, info.repo_name, info.branch)
        }
        None => Context::new(note),
    };

    // Save the context
    storage.save_context(&context)?;

    println!("✅ Context saved: \"{}\"", context.note);

    Ok(())
}
