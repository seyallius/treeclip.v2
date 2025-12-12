use crate::commands::run::args::RunArgs;
use crate::core::traversal::filter;
use crate::core::utils;
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
        utils::validate_path_exists(&run_args.input_path)?;
        log_starting_path(&run_args.input_path);
        self.traverse(run_args.skip_hidden, run_args.verbose)?;
        println!("✅ Extraction complete");
        Ok(())
    }

    fn traverse(&self, skip_hidden: bool, verbose: bool) -> anyhow::Result<()> {
        let walker = WalkDir::new(&self.root).into_iter().filter_entry(|entry| {
            let non_excluded_path = !filter::should_exclude(entry.path(), &self.exclude_patterns);
            let non_hidden_path = !skip_hidden || !filter::is_hidden(entry);
            non_excluded_path && non_hidden_path
        });

        let mut file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.output)?;
        let mut first = true;
        for entry in walker.filter_map(|e| e.ok()) {
            let entry_path = entry.path();

            // skip reading output itself
            if entry_path.eq(&self.output) {
                continue;
            }

            if entry_path.is_file() {
                if verbose {
                    // TODO: do some verbose thingy
                    println!("📄 {}", entry_path.display());
                }

                let relative_path = entry_path.strip_prefix(&self.root).unwrap_or(entry_path);

                if !first {
                    writeln!(file)?;
                }

                // Write the header: ==> relative/path
                writeln!(file, "==> {}", relative_path.display())
                    .context("failed to write path header")?;

                // Read and write content
                let content = fs::read_to_string(entry_path) //TODO: switch to buffered streaming (BufReader::read_line or copy) later — but only if you want extra polish.
                    .with_context(|| {
                        format!("failed writing content for {}", entry_path.display())
                    })?;
                file.write_all(content.trim_end().as_bytes())
                    .context("failed to write content to file")?;

                // Add new line between files
                writeln!(file)?;
                first = false;
            }
        }
        Ok(())
    }
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

#[cfg(test)]
mod walker_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

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
        println!("\n beg----");
        print!("Output content:\n{}", output_content); // Debug
        println!("\n end----");

        // Check format more precisely
        let expected = "\
==> subdir/file2.txt
Content of file 2

==> file1.txt
Content of file 1
"; // <-- this is new line

        assert_eq!(output_content, expected);

        Ok(())
    }
}
