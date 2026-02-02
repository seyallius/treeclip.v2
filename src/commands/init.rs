//! init - Handles initialization of .treeclipignore configuration file.

use crate::core::errors::FileSystemError;
use crate::core::ui::messages::Messages;
use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Default ignore patterns to include in .treeclipignore.
const DEFAULT_PATTERNS: &[&str] = &[
    "# treeclip temporary files",
    "treeclip_temp*.txt",
    ".treeclipignore",
    "",
    "# Build outputs",
    "target/",
    "*.exe",
    "*.dll",
    "*.so",
    "*.dylib",
    "",
    "# Dependencies",
    "node_modules/",
    "vendor/",
    "",
    "# IDE files",
    ".idea/",
    ".vscode/",
    "*.swp",
    "*.swo",
    "*~",
    "",
    "# OS files",
    ".DS_Store",
    "Thumbs.db",
];

/// Standard ignore file names to check for importing patterns.
const STANDARD_IGNORE_FILES: &[&str] = &[
    ".gitignore",
    ".dockerignore",
    ".npmignore",
    ".eslintignore",
];

/// Initializes a .treeclipignore file in the specified directory.
pub struct InitCommand {
    target_dir: PathBuf,
    force: bool,
}

impl InitCommand {
    // -------------------------------------------- Public Functions --------------------------------------------

    /// Creates a new InitCommand instance.
    ///
    /// # Arguments
    ///
    /// * `target_dir` - Directory where .treeclipignore will be created
    /// * `force` - Whether to overwrite existing .treeclipignore without prompting
    pub fn new(target_dir: PathBuf, force: bool) -> Self {
        Self { target_dir, force }
    }

    /// Executes the init command to create or update .treeclipignore file.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the file was successfully created/updated, or an error if:
    /// - User declined to overwrite existing file
    /// - File operations failed
    /// - Directory is not writable
    pub fn execute(&self) -> Result<()> {
        println!("{}", Messages::init_starting());

        let treeclip_ignore_path = self.target_dir.join(".treeclipignore");

        // Check if .treeclipignore already exists.
        if treeclip_ignore_path.exists() {
            if !self.force && !self.confirm_overwrite()? {
                println!("{}", Messages::init_cancelled());
                return Ok(());
            }
            println!("{}", Messages::init_overwriting());
        }

        // Collect patterns from existing ignore files.
        let imported_patterns = self.collect_patterns_from_ignore_files()?;

        // Write the .treeclipignore file.
        self.write_treeclip_ignore(&treeclip_ignore_path, &imported_patterns)?;

        println!("{}", Messages::init_success(&treeclip_ignore_path));

        // Show summary of what was done.
        self.print_summary(&imported_patterns);

        Ok(())
    }

    // -------------------------------------------- Private Helper Functions --------------------------------------------

    /// Prompts the user for confirmation to overwrite existing .treeclipignore.
    fn confirm_overwrite(&self) -> Result<bool> {
        println!("{}", Messages::init_file_exists_warning());
        print!("Do you want to overwrite it? (y/N): ");
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        let response = response.trim().to_lowercase();
        Ok(response == "y" || response == "yes")
    }

    /// Collects patterns from standard ignore files in the target directory.
    fn collect_patterns_from_ignore_files(&self) -> Result<Vec<ImportedPattern>> {
        let mut imported_patterns = Vec::new();

        for ignore_file_name in STANDARD_IGNORE_FILES {
            let ignore_path = self.target_dir.join(ignore_file_name);

            if ignore_path.exists() {
                match self.read_patterns_from_file(&ignore_path) {
                    Ok(patterns) => {
                        if !patterns.is_empty() {
                            println!(
                                "{}",
                                Messages::init_importing_from(ignore_file_name, patterns.len())
                            );
                            imported_patterns.push(ImportedPattern {
                                source: ignore_file_name.to_string(),
                                patterns,
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "{}",
                            Messages::init_import_warning(ignore_file_name, &e.to_string())
                        );
                    }
                }
            }
        }

        Ok(imported_patterns)
    }

    /// Reads patterns from a single ignore file, filtering comments and empty lines.
    fn read_patterns_from_file(&self, path: &Path) -> Result<Vec<String>> {
        let file = File::open(path).map_err(|e| FileSystemError::ReadFailed {
            path: path.to_path_buf(),
            source: e,
        })?;

        let reader = BufReader::new(file);
        let mut patterns = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            // Skip empty lines and comments.
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                patterns.push(line);
            }
        }

        Ok(patterns)
    }

    /// Writes the .treeclipignore file with default patterns and imported patterns.
    fn write_treeclip_ignore(
        &self,
        path: &Path,
        imported_patterns: &[ImportedPattern],
    ) -> Result<()> {
        let mut file = File::create(path)
            .map_err(|e| FileSystemError::WriteFailed {
                path: path.to_path_buf(),
                source: e,
            })
            .with_context(|| format!("Failed to create .treeclipignore at {}", path.display()))?;

        // Write header.
        writeln!(
            file,
            "# .treeclipignore - treeclip ignore patterns configuration"
        )?;
        writeln!(file, "# Generated by treeclip init")?;
        writeln!(file)?;

        // Write default patterns.
        for pattern in DEFAULT_PATTERNS {
            writeln!(file, "{}", pattern)?;
        }

        // Write imported patterns if any exist.
        if !imported_patterns.is_empty() {
            writeln!(file)?;
            writeln!(file, "# ==================== Imported Patterns ====================")?;

            // Use HashSet to track already written patterns (avoid duplicates).
            let mut written_patterns = HashSet::new();

            // Add default patterns to the set.
            for pattern in DEFAULT_PATTERNS {
                if !pattern.is_empty() && !pattern.starts_with('#') {
                    written_patterns.insert(pattern.to_string());
                }
            }

            for imported in imported_patterns {
                writeln!(file)?;
                writeln!(file, "# From {}", imported.source)?;

                for pattern in &imported.patterns {
                    let trimmed = pattern.trim();

                    // Only write if not a duplicate.
                    if !written_patterns.contains(trimmed) {
                        writeln!(file, "{}", pattern)?;
                        written_patterns.insert(trimmed.to_string());
                    }
                }
            }
        }

        file.flush()?;
        Ok(())
    }

    /// Prints a summary of the initialization operation.
    fn print_summary(&self, imported_patterns: &[ImportedPattern]) {
        println!("\n{}", "Summary:".bright_cyan().bold());
        println!("  {} Default patterns added", "✓".bright_green());

        if imported_patterns.is_empty() {
            println!(
                "  {} No existing ignore files found to import",
                "ℹ".bright_blue()
            );
        } else {
            let total_imported: usize = imported_patterns.iter().map(|p| p.patterns.len()).sum();
            println!(
                "  {} Imported {} patterns from {} file(s)",
                "✓".bright_green(),
                total_imported,
                imported_patterns.len()
            );

            for imported in imported_patterns {
                println!(
                    "    • {} ({} patterns)",
                    imported.source.bright_blue(),
                    imported.patterns.len()
                );
            }
        }

        println!(
            "\n{} You can now edit .treeclipignore to customize ignore patterns.",
            "💡".bright_yellow()
        );
    }
}

// -------------------------------------------- Helper Types --------------------------------------------

/// Represents patterns imported from a specific ignore file.
struct ImportedPattern {
    source: String,
    patterns: Vec<String>,
}

// -------------------------------------------- Tests --------------------------------------------

#[cfg(test)]
mod init_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let init_cmd = InitCommand::new(temp_dir.path().to_path_buf(), false);

        init_cmd.execute()?;

        let treeclip_ignore_path = temp_dir.path().join(".treeclipignore");
        assert!(treeclip_ignore_path.exists());

        let content = fs::read_to_string(&treeclip_ignore_path)?;
        assert!(content.contains("treeclip_temp*.txt"));
        assert!(content.contains("target/"));
        assert!(content.contains("*.exe"));

        Ok(())
    }

    #[test]
    fn test_init_imports_gitignore() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Create a .gitignore with some patterns.
        let gitignore_path = temp_dir.path().join(".gitignore");
        fs::write(
            &gitignore_path,
            "# Comment\nnode_modules\n*.log\n\n# Another comment\ndist/\n",
        )?;

        let init_cmd = InitCommand::new(temp_dir.path().to_path_buf(), false);
        init_cmd.execute()?;

        let treeclip_ignore_path = temp_dir.path().join(".treeclipignore");
        let content = fs::read_to_string(&treeclip_ignore_path)?;

        assert!(content.contains("node_modules"));
        assert!(content.contains("*.log"));
        assert!(content.contains("dist/"));
        assert!(content.contains("From .gitignore"));

        Ok(())
    }

    #[test]
    fn test_init_avoids_duplicates() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Create a .gitignore with patterns that overlap with defaults.
        let gitignore_path = temp_dir.path().join(".gitignore");
        fs::write(&gitignore_path, "target/\nnode_modules/\n*.exe\n")?;

        let init_cmd = InitCommand::new(temp_dir.path().to_path_buf(), false);
        init_cmd.execute()?;

        let treeclip_ignore_path = temp_dir.path().join(".treeclipignore");
        let content = fs::read_to_string(&treeclip_ignore_path)?;

        // Count occurrences - should only appear once.
        let target_count = content.matches("target/").count();
        assert_eq!(target_count, 1, "target/ should appear only once");

        Ok(())
    }

    #[test]
    fn test_init_imports_multiple_files() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Create multiple ignore files.
        fs::write(temp_dir.path().join(".gitignore"), "*.log\n")?;
        fs::write(temp_dir.path().join(".dockerignore"), "*.tmp\n")?;

        let init_cmd = InitCommand::new(temp_dir.path().to_path_buf(), false);
        init_cmd.execute()?;

        let treeclip_ignore_path = temp_dir.path().join(".treeclipignore");
        let content = fs::read_to_string(&treeclip_ignore_path)?;

        assert!(content.contains("From .gitignore"));
        assert!(content.contains("From .dockerignore"));
        assert!(content.contains("*.log"));
        assert!(content.contains("*.tmp"));

        Ok(())
    }

    #[test]
    fn test_read_patterns_filters_comments_and_empty_lines() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_file = temp_dir.path().join("test.ignore");

        fs::write(
            &test_file,
            "# Comment line\npattern1\n\n# Another comment\npattern2\n   \n",
        )?;

        let init_cmd = InitCommand::new(temp_dir.path().to_path_buf(), false);
        let patterns = init_cmd.read_patterns_from_file(&test_file)?;

        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0], "pattern1");
        assert_eq!(patterns[1], "pattern2");

        Ok(())
    }
}
