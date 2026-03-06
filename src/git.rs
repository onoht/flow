use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Git information for the current directory
pub struct GitInfo {
    pub repo_name: String,
    pub branch: String,
}

/// Detect git information for the current directory
pub fn detect_git_info(path: &Path) -> Result<Option<GitInfo>> {
    // Check if we're in a git repo
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(path)
        .output();

    let git_dir = match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => return Ok(None), // Not in a git repo
    };

    // Get the repository root directory name
    let root_output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output();

    let repo_name = match root_output {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let root = Path::new(stdout.trim());
            root.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        }
        _ => "unknown".to_string(),
    };

    // Get the current branch
    let branch_output = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(path)
        .output();

    let branch = match branch_output {
        Ok(o) if o.status.success() => {
            let b = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if b.is_empty() {
                // Detached HEAD or unborn branch - try to get HEAD ref
                get_head_branch(&git_dir, path)
            } else {
                b
            }
        }
        _ => "master".to_string(),
    };

    Ok(Some(GitInfo { repo_name, branch }))
}

/// Get branch from .git/HEAD (for detached HEAD or unborn branches)
fn get_head_branch(git_dir: &str, path: &Path) -> String {
    use std::fs;

    // Resolve .git directory (could be in worktree or submodule)
    let git_path = if Path::new(git_dir).is_absolute() {
        PathBuf::from(git_dir)
    } else {
        path.join(git_dir)
    };

    let head_path = git_path.join("HEAD");
    if let Ok(content) = fs::read_to_string(&head_path) {
        let content = content.trim();
        // Check if it's a ref: refs/heads/branch-name
        if let Some(branch) = content.strip_prefix("ref: refs/heads/") {
            return branch.to_string();
        }
    }
    "master".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    fn git_available() -> bool {
        Command::new("git").arg("--version").output().is_ok()
    }

    fn init_git_repo(path: &Path) {
        Command::new("git")
            .args(["init"])
            .current_dir(path)
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(path)
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(path)
            .output()
            .unwrap();
    }

    #[test]
    fn test_detect_git_info_in_repo() {
        if !git_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();
        init_git_repo(path);

        // Create a test file and commit
        fs::write(path.join("test.txt"), "test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(path)
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(path)
            .output()
            .unwrap();

        let info = detect_git_info(path).unwrap();
        assert!(info.is_some());
        let info = info.unwrap();
        // Branch name depends on git config (init.defaultBranch)
        assert!(matches!(info.branch.as_str(), "master" | "main"));
    }

    #[test]
    fn test_detect_git_info_not_in_repo() {
        if !git_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        let info = detect_git_info(temp_dir.path()).unwrap();
        assert!(info.is_none());
    }

    #[test]
    fn test_detect_git_info_unborn_branch() {
        if !git_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Initialize a git repo without any commits (unborn branch)
        init_git_repo(path);

        let info = detect_git_info(path).unwrap();
        assert!(info.is_some());
        let info = info.unwrap();
        // Should detect the default branch name from .git/HEAD
        assert!(matches!(info.branch.as_str(), "master" | "main"));
    }
}
