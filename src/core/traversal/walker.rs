use crate::commands::run::args::RunArgs;
use anyhow::Context;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Walker {
    root: PathBuf,
    output: PathBuf,
    exclude_patterns: Vec<String>,
}

impl Walker {
    pub fn new(root: &PathBuf, output: &PathBuf, exclude_patterns: &Vec<String>) -> Self {
        Self {
            root: root.clone(),
            output: output.clone(),
            exclude_patterns: exclude_patterns.clone(),
        }
    }
}

impl Walker {
    pub fn process_dir(&self, run_args: &RunArgs) -> anyhow::Result<()> {
        validate_path_exists(&run_args.input_path)?;
        log_starting_path(&run_args.input_path);
        self.traverse(run_args.skip_hidden, run_args.verbose)?;
        println!("✅ Extraction complete");
        Ok(())
    }

    fn traverse(&self, skip_hidden: bool, verbose: bool) -> anyhow::Result<()> {
        let walker = WalkDir::new(&self.root).into_iter().filter_entry(|entry| {
            let non_excluded_path = !should_exclude(entry.path(), &self.exclude_patterns);
            let non_hidden_path = !skip_hidden || !is_hidden(entry);
            non_excluded_path && non_hidden_path
        });

        let mut file = File::options()
            .append(true)
            .create(true)
            .open(&self.output)?;
        for entry in walker.filter_map(|e| e.ok()) {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if verbose {
                    // TODO: do some verbose thingy
                    println!("📄 {}", entry_path.display());
                }

                let relative_path = entry_path.strip_prefix(&self.root).unwrap_or(entry_path);

                // Write the header: ==> relative/path
                writeln!(file, "==> {}", relative_path.display())
                    .context("failed to write path header")?;

                // Read and write content
                let content = fs::read_to_string(entry_path)
                    .context(format!("reading file {} failed", entry_path.display()))?;
                file.write_all(b"\n")
                    .context("failed to write new_line to file")?;
                file.write_all(content.as_bytes())
                    .context("failed to write content to file")?;

                // Add new line between files
                writeln!(file)?;
            }
        }

        Ok(())
    }
}

fn validate_path_exists(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }
    Ok(())
}

fn log_starting_path(path: &Path) {
    if path == Path::new(".") {
        if let Ok(cwd) = std::env::current_dir() {
            println!("Traversing directory: {}", cwd.display());
        }
    } else {
        println!("Traversing directory: {}", path.display());
    }
}

fn should_exclude(path: &Path, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return false;
    }

    let path_str = path.to_string_lossy().to_lowercase();
    patterns
        .iter()
        .any(|pattern| path_str.contains(&pattern.to_lowercase()))
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|str| {
            let hidden_entry = str.starts_with(".");
            if hidden_entry {
                println!("Hidden entry '{}' was skipped", entry.path().display());
            }
            hidden_entry
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validate_path_exists_valid() {
        let temp_dir = TempDir::new().unwrap();
        let result = validate_path_exists(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_exists_invalid() {
        let result = validate_path_exists(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_should_exclude() {
        let path = Path::new("/home/user/project/node_modules/package");
        let patterns = vec!["node_modules".to_string(), ".git".to_string()];

        assert!(should_exclude(path, &patterns));

        let path2 = Path::new("/home/user/project/src/main.rs");
        assert!(!should_exclude(path2, &patterns));
    }

    #[test]
    fn test_should_exclude_case_insensitive() {
        let path = Path::new("/home/user/project/NODE_MODULES/package");
        let patterns = vec!["node_modules".to_string()];

        assert!(should_exclude(path, &patterns));
    }

    #[test]
    fn test_is_hidden() {
        // Create a mock entry
        let temp_dir = TempDir::new().unwrap();
        let hidden_file = temp_dir.path().join(".hidden");
        fs::write(&hidden_file, "").unwrap();

        let entry = walkdir::WalkDir::new(temp_dir.path())
            .into_iter()
            .next()
            .unwrap()
            .unwrap();

        assert!(is_hidden(&entry));
    }

    #[test]
    fn test_traverse_directory() {
        let temp_dir = TempDir::new().unwrap();
        let mut path = Path::new(temp_dir.path()).to_path_buf();
        path.push("output");
        path.set_extension("txt");
        assert_eq!(temp_dir.path(), path.parent().unwrap());

        let walker = Walker::new(&temp_dir.path().to_path_buf(), &path, &vec![]);
        let result = walker.traverse(false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_traverse_directory_writes_correct_format() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;

        // Create test files
        let file1_path = temp_dir.path().join("file1.txt");
        fs::write(&file1_path, "Content of file 1")?;

        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir)?;
        let file2_path = subdir.join("file2.txt");
        fs::write(&file2_path, "Content of file 2")?;

        let output_path = temp_dir.path().join("output.txt");

        // Run traversal
        let walker = Walker::new(&temp_dir.path().to_path_buf(), &output_path, &vec![]);
        walker.traverse(false, false)?;

        // Read and verify output
        let output_content = fs::read_to_string(&output_path)?;
        println!("Output content:\n{}", output_content); // Debug

        // Check format more precisely
        let expected = "\
==> file1.txt
Content of file 1

==> subdir/file2.txt
Content of file 2

";

        // Compare normalized output (trim trailing whitespace differences)
        assert_eq!(output_content.trim_end(), expected.trim_end());

        Ok(())
    }
}
