use anyhow::Result;

pub fn run(limit: usize) -> Result<()> {
    println!("📊 Context History (last {} entries)", limit);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("History tracking not yet implemented.");
    println!();
    println!("This feature will be added in a future version.");
    println!("For now, only the current context is tracked.");

    Ok(())
}
