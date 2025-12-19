use crate::core::ui::messages::Messages;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

pub struct ExcludeMatcher {
    inner: Gitignore,
}

impl ExcludeMatcher {
    pub fn new(root: &Path, cli_patterns: &[String]) -> anyhow::Result<Self> {
        let mut builder = GitignoreBuilder::new(root);

        // 1. .treeclipignore file (optional)
        let ignore_file = root.join(".treeclipignore");
        //TODO: path is said to be concurrent non-safe = [https://doc.rust-lang.org/stable/std/fs/index.html#:~:text=For%20example%2C%20checking%20if%20a%20file%20exists%20and%20then%20creating%20it%20if%20it%20doesn%E2%80%99t%20is%20vulnerable%20to%20TOCTOU%20%2D%20another%20process%20could%20create%20the%20file%20between%20your%20check%20and%20creation%20attempt]
        if ignore_file.exists() {
            println!(
                "{}",
                Messages::found_ignore_file(&ignore_file.display().to_string())
            );
            println!("{}", Messages::applying_ignore_rules());
            builder.add(ignore_file);
        }

        // 2. CLI patterns
        if !cli_patterns.is_empty() {
            for pat in cli_patterns {
                builder.add_line(None, pat)?;
            }
        }

        let inner = builder.build()?;
        Ok(Self { inner })
    }
}

impl ExcludeMatcher {
    pub fn is_excluded(&self, path: &Path) -> bool {
        self.inner.matched(path, path.is_dir()).is_ignore()
    }
}

#[cfg(test)]
mod exclude_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_excluded() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        let node_modules = root.join("node_modules");
        fs::create_dir(&node_modules)?;

        let ignore_file = root.join(".treeclipignore");
        fs::write(&ignore_file, "node_modules")?; // should be excluded

        let temp1 = root.join("temp1");
        fs::write(&temp1, "temp1")?;

        let temp2 = root.join("temp2");
        fs::write(&temp2, "temp2")?;

        let matcher = ExcludeMatcher::new(root, &[])?;

        let is_not_excluded = !matcher.is_excluded(root);
        assert!(is_not_excluded);

        let is_not_excluded = !matcher.is_excluded(&temp1);
        assert!(is_not_excluded);

        let is_not_excluded = !matcher.is_excluded(&temp2);
        assert!(is_not_excluded);

        let is_excluded = matcher.is_excluded(&node_modules);
        assert!(is_excluded); // should be excluded

        Ok(())
    }
}
