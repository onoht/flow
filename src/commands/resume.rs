use crate::storage::Storage;
use anyhow::Result;

pub fn run() -> Result<()> {
    let storage = Storage::default_location()?;

    match storage.load_context()? {
        None => {
            println!("No current context to resume.");
            println!("Use 'flow note \"what you're working on\"' to set one.");
        }
        Some(context) => {
            println!("🎯 Resume Your Work");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!();

            if let (Some(repo), Some(branch)) = (&context.repo, &context.branch) {
                println!("Project: {} (branch: {})", repo, branch);
                println!();
            }

            println!("You were working on:");
            println!("  💭 \"{}\"", context.note);
            println!();

            println!("Quick actions:");
            println!("  • flow note \"...\"  - update what you're doing");
            println!("  • flow done        - mark complete, start fresh");
            println!("  • flow status      - see full context details");
        }
    }

    Ok(())
}
