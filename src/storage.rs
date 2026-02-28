use crate::context::Context;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Handles persistence of context data
pub struct Storage {
    /// Base directory for storing data
    base_dir: PathBuf,
}

impl Storage {
    /// Create a new storage instance
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Get the default storage location (~/.flow)
    pub fn default_location() -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(Self::new(home.join(".flow")))
    }

    /// Get the path to the current context file
    fn context_file(&self) -> PathBuf {
        self.base_dir.join("context.json")
    }

    /// Ensure the storage directory exists
    fn ensure_dir(&self) -> Result<()> {
        if !self.base_dir.exists() {
            fs::create_dir_all(&self.base_dir)?;
        }
        Ok(())
    }

    /// Save the current context
    pub fn save_context(&self, context: &Context) -> Result<()> {
        self.ensure_dir()?;
        let path = self.context_file();
        let json = serde_json::to_string_pretty(context)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load the current context
    pub fn load_context(&self) -> Result<Option<Context>> {
        let path = self.context_file();
        if !path.exists() {
            return Ok(None);
        }

        let json = fs::read_to_string(path)?;
        let context: Context = serde_json::from_str(&json)?;
        Ok(Some(context))
    }

    /// Clear the current context
    pub fn clear_context(&self) -> Result<()> {
        let path = self.context_file();
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_context() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let context = Context::new("testing auth bug".to_string());
        storage.save_context(&context).unwrap();

        let loaded = storage.load_context().unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.note, "testing auth bug");
    }

    #[test]
    fn test_load_nonexistent_context() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let loaded = storage.load_context().unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_clear_context() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let context = Context::new("testing".to_string());
        storage.save_context(&context).unwrap();

        storage.clear_context().unwrap();
        let loaded = storage.load_context().unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_save_creates_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let non_existent_dir = temp_dir.path().join("nested").join("dirs");
        let storage = Storage::new(non_existent_dir.clone());

        // Directory doesn't exist yet
        assert!(!non_existent_dir.exists());

        let context = Context::new("test".to_string());
        storage.save_context(&context).unwrap();

        // Directory should be created
        assert!(non_existent_dir.exists());
    }

    #[test]
    fn test_context_with_special_characters() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let note = "Testing with special chars: émojis 🎉, quotes \"'\", newlines\n, and tabs\t";
        let context = Context::new(note.to_string());
        storage.save_context(&context).unwrap();

        let loaded = storage.load_context().unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.note, note);
    }
}
