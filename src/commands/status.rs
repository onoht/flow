use crate::storage::Storage;
use anyhow::Result;
use chrono::Utc;

pub fn run() -> Result<()> {
    let storage = Storage::default_location()?;

    match storage.load_context()? {
        None => {
            println!("No current context set.");
            println!("Use 'flow note \"what you're working on\"' to set one.");
        }
        Some(context) => {
            println!("📍 Current Context");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!();

            if let (Some(repo), Some(branch)) = (&context.repo, &context.branch) {
                println!("Project: {} (branch: {})", repo, branch);
                println!();
            }

            println!("💭 \"{}\"", context.note);

            // Calculate time ago
            let now = Utc::now();
            let duration = now.signed_duration_since(context.timestamp);
            let time_ago = if duration.num_minutes() < 1 {
                "just now".to_string()
            } else if duration.num_minutes() < 60 {
                format!("{} minutes ago", duration.num_minutes())
            } else if duration.num_hours() < 24 {
                format!("{} hours ago", duration.num_hours())
            } else {
                format!("{} days ago", duration.num_days())
            };

            println!("   Last updated: {}", time_ago);
            println!();

            if duration.num_minutes() > 30 {
                println!("💡 Resume with: flow resume");
            }
        }
    }

    Ok(())
}
