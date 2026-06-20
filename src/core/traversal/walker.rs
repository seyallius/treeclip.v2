//! walker - Handles directory traversal and file content extraction operations.

use crate::commands::args::RunArgs;
use crate::core::errors::{FileSystemError, TraversalError};
use crate::core::traversal::filter;
use crate::core::ui::animations;
use crate::core::{exclude, tree, ui, utils};
use anyhow::Context;
use colored::Colorize;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Walker handles directory traversal and content extraction to a single output file.
pub struct Walker {
    root: PathBuf,
    inputs: Vec<PathBuf>,
    output: PathBuf,
}

impl Walker {
    /// Creates a new Walker instance with the specified configuration.
    pub fn new(root: &Path, inputs: &[PathBuf], output: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            inputs: inputs.to_owned(),
            output: output.to_path_buf(),
        }
    }

    /// Processes the directory based on the provided run arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Input path does not exist
    /// - Traversal fails
    /// - Output file cannot be written
    pub fn process_dir(&self, run_args: &RunArgs) -> anyhow::Result<()> {
        // Validate that the input path exists (this is the current walker's input path)
        for input in &self.inputs {
            utils::validate_path_exists(input)
                .with_context(|| format!("Input path validation failed: {}", input.display()))?;
        }

        self.traverse(run_args)
            .with_context(|| format!("Directory traversal failed for: {:?}", self.inputs))?;

        if run_args.verbose {
            println!(
                "\n{} {}",
                "🎊".green(),
                "Extraction complete! All files gathered~".bright_green()
            );
        }
        Ok(())
    }
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

impl Walker {
    /// Traverses the directory tree and writes file contents to the output file.
    fn traverse(&self, run_args: &RunArgs) -> anyhow::Result<()> {
        let matcher = exclude::ExcludeMatcher::new(&self.root, run_args).with_context(|| {
            format!(
                "Failed to create exclusion matcher for root: {}",
                self.root.display()
            )
        })?;

        // Collect all file paths to process
        let file_paths: Vec<PathBuf> = self
            .inputs
            .iter()
            .flat_map(|input| {
                WalkDir::new(input).into_iter().filter_entry(|entry| {
                    let excluded_by_ignore_rules = matcher.is_excluded(entry.path()); // Check ignore files first
                    let skip_hidden_flag_active = run_args.skip_hidden; // Check the flag
                    let is_current_entry_hidden = filter::is_hidden(entry, run_args.verbose); // Check if hidden

                    // Include the entry if:
                    // 1. It's NOT excluded by ignore rules (e.g., .treeclipignore, .gitignore)
                    // AND
                    // 2. Either the --skip-hidden flag is OFF, OR the entry is NOT hidden
                    !excluded_by_ignore_rules
                        && (!skip_hidden_flag_active || !is_current_entry_hidden)
                })
            })
            .filter_map(|entry| {
                entry
                    .map_err(|e| {
                        eprintln!("Error accessing directory entry: {:?}", e);
                    })
                    .ok()
                    .filter(|e| e.path() != self.output) // Skip reading output itself
                    .filter(|e| e.path().is_file())
                    .map(|e| e.path().to_path_buf())
            })
            .collect();

        // Track which paths were actually traversed
        let traversed_paths = file_paths.clone();

        // Check if any files were found
        if file_paths.is_empty()
            && let Some(input) = self.inputs.first()
        {
            return Err(TraversalError::NoFilesFound(input.to_path_buf()).into());
        }

        // Process files in parallel using rayon
        let file_contents: Vec<anyhow::Result<(PathBuf, String)>> = file_paths
            .into_par_iter()
            .map(|path| {
                // Read file content
                let content = fs::read_to_string(&path)
                    .map_err(|e| FileSystemError::ReadFailed {
                        path: path.clone(),
                        source: e,
                    })
                    .with_context(|| {
                        format!("Failed to read file contents from: {}", path.display())
                    })?;

                Ok((path, content))
            })
            .collect();

        // Write all contents to output file sequentially to maintain order
        // TODO: Consider using BufWriter for better I/O performance on large outputs
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.output)
            .map_err(|e| FileSystemError::WriteFailed {
                path: self.output.clone(),
                source: e,
            })
            .with_context(|| {
                format!(
                    "Failed to create or open output file: {}",
                    self.output.display()
                )
            })?;

        let mut file_count = 0;
        let mut first = true; // Only true for first traversal

        let tree_emojis = vec!["🌱", "🌿", "🍃", "🌳", "🌲", "🎄"];

        // Write the processed content to the output file
        for result in file_contents {
            let (entry_path, content) = result?;
            file_count += 1;

            // Progress indicator (only in verbose mode and not fast mode)
            if run_args.verbose
                && !run_args.fast_mode
                && file_count % 5 == 0
                && let Some(msg) = animations::progress_counter(&tree_emojis, file_count, 5)
            {
                print!("\r{msg}");
                stdout().flush().with_context(|| "Failed to flush stdout")?;
            }

            self.write_file_content_with_content(&mut file, &entry_path, &content, &mut first)
                .with_context(|| {
                    format!("Failed to write content for file: {}", entry_path.display())
                })?;
        }

        // Add tree structure if requested
        if run_args.tree {
            println!("\n{}\n", ui::messages::Messages::tree_structure_enabled());

            writeln!(file)?;
            writeln!(file, "Directory structure:")?;

            let mut tree_state = tree::TreeState::new();
            // Build tree only from traversed paths instead of all input paths
            tree::TreeState::write_tree_from_paths(
                &traversed_paths,
                &self.root,
                &mut file,
                &mut tree_state,
            )?;
        }

        if run_args.verbose {
            println!(
                "\r{} Collected {} files from {:?}! {}",
                "✨".green(),
                file_count,
                self.inputs,
                "Nice work!".bright_green()
            );
        }

        Ok(())
    }

    /// Writes a file's content to the output file with proper formatting, using pre-read content.
    fn write_file_content_with_content(
        &self,
        output_file: &mut File,
        entry_path: &Path,
        content: &str,
        first: &mut bool,
    ) -> anyhow::Result<()> {
        let relative_path = entry_path.strip_prefix(&self.root).unwrap_or(entry_path);

        if !*first {
            writeln!(output_file)
                .map_err(|e| FileSystemError::WriteFailed {
                    path: self.output.clone(),
                    source: e,
                })
                .with_context(|| {
                    format!(
                        "Failed to write newline separator to: {}",
                        self.output.display()
                    )
                })?;
        }

        // Write the header: ==> relative/path
        writeln!(output_file, "==> {}", relative_path.display())
            .map_err(|e| FileSystemError::WriteFailed {
                path: self.output.clone(),
                source: e,
            })
            .with_context(|| {
                format!(
                    "Failed to write path header for: {}",
                    relative_path.display()
                )
            })?;

        output_file
            .write_all(content.trim_end().as_bytes())
            .map_err(|e| FileSystemError::WriteFailed {
                path: self.output.clone(),
                source: e,
            })
            .with_context(|| {
                format!(
                    "Failed to write file content to output: {}",
                    self.output.display()
                )
            })?;

        // Add newline between files
        writeln!(output_file)
            .map_err(|e| FileSystemError::WriteFailed {
                path: self.output.clone(),
                source: e,
            })
            .with_context(|| "Failed to write trailing newline to output file")?;

        *first = false;

        Ok(())
    }
}

#[cfg(test)]
mod walker_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_walker_creation() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("output.txt");

        let walker = Walker::new(
            temp_dir.path(),
            &vec![temp_dir.path().to_path_buf()],
            &output,
        );

        assert_eq!(walker.root, temp_dir.path());
        // assert_eq!(walker.inputs, temp_dir.path());
        assert_eq!(walker.output, output);
    }

    #[test]
    fn test_traverse_creates_output_file() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let output = temp_dir.path().join("output.txt");

        // Create a test file
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content")?;

        let walker = Walker::new(
            temp_dir.path(),
            &vec![temp_dir.path().to_path_buf()],
            &output,
        );

        let args = RunArgs {
            input_paths: vec![temp_dir.path().to_path_buf()],
            output_path: Some(output.clone()),
            root: Some(temp_dir.path().to_path_buf()),
            exclude: vec![],
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: false,
            no_skip_hidden: false,
            raw: true,
            fast_mode: true,
            tree: false,
        };

        walker.traverse(&args)?;

        assert!(output.exists());
        Ok(())
    }

    #[test]
    fn test_traverse_writes_correct_format() -> anyhow::Result<()> {
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
        let walker = Walker::new(
            temp_dir.path(),
            &vec![temp_dir.path().to_path_buf()],
            &output_path,
        );

        let args = RunArgs {
            input_paths: vec![temp_dir.path().to_path_buf()],
            output_path: Some(output_path.clone()),
            root: Some(temp_dir.path().to_path_buf()),
            exclude: vec![],
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: false,
            no_skip_hidden: false,
            raw: true,
            fast_mode: true,
            tree: false,
        };

        walker.traverse(&args)?;

        // Read and verify output
        let output_content = fs::read_to_string(&output_path)?;

        // Verify format (order may vary based on filesystem)
        assert!(output_content.contains("==> file1.txt") || output_content.contains("==> subdir"));
        assert!(output_content.contains("Content of file 1"));
        assert!(
            output_content.contains("==> subdir/file2.txt")
                || output_content.contains("==> subdir\\file2.txt")
        );
        assert!(output_content.contains("Content of file 2"));

        Ok(())
    }

    #[test]
    fn test_process_dir_validates_path() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("output.txt");

        let walker = Walker::new(
            temp_dir.path(),
            &vec![PathBuf::from("/nonexistent/path").to_path_buf()],
            &output,
        );

        let args = RunArgs {
            input_paths: vec![PathBuf::from("/nonexistent/path")],
            output_path: Some(output),
            root: Some(temp_dir.path().to_path_buf()),
            exclude: vec![],
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: true,
            no_skip_hidden: false,
            raw: true,
            fast_mode: true,
            tree: false,
        };

        let result = walker.process_dir(&args);
        assert!(result.is_err());

        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("does not exist") || error_msg.contains("validation failed"));
    }

    #[test]
    fn test_no_files_found_error() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let output = temp_dir.path().join("output.txt");

        // Create an empty directory
        let empty_dir = temp_dir.path().join("empty");
        fs::create_dir(&empty_dir)?;

        let walker = Walker::new(temp_dir.path(), &vec![empty_dir.clone()], &output);

        let args = RunArgs {
            input_paths: vec![empty_dir.clone()],
            output_path: Some(output),
            root: Some(temp_dir.path().to_path_buf()),
            exclude: vec![],
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: false,
            no_skip_hidden: false,
            raw: true,
            fast_mode: true,
            tree: false,
        };

        let result = walker.traverse(&args);
        assert!(result.is_err());

        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("No files found"));

        Ok(())
    }

    #[test]
    fn test_traverse_walker_ignores_wildcard() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let output = temp_dir.path().join("output.txt");

        let main = temp_dir.path().join("main.rs");
        fs::write(&main, "fn main() { println!(\"Hello world\"); }")?;
        let main_test = temp_dir.path().join("main_test.rs");
        fs::write(
            &main_test,
            "fn test_main() { println!(\"Test hello world\"); }",
        )?;
        let go = temp_dir.path().join("main.go");
        fs::write(
            &go,
            "Package main import \"fmt\" func main() { fmt.Println(\"Hello world\") }",
        )?;

        let exclude_patterns = vec!["*_test.rs".to_string(), "*.go".to_string()];
        let walker = Walker::new(
            temp_dir.path(),
            &vec![temp_dir.path().to_path_buf()],
            &output,
        );
        let args = RunArgs {
            input_paths: vec![temp_dir.path().to_path_buf()],
            output_path: Some(output.to_path_buf()),
            root: Some(temp_dir.path().to_path_buf()),
            exclude: exclude_patterns,
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: false,
            no_skip_hidden: false,
            raw: true,
            fast_mode: true,
            tree: false,
        };

        let result = walker.traverse(&args);
        assert!(result.is_ok());

        // Read and verify output
        let output_content = fs::read_to_string(&output)?;

        assert!(output_content.contains("==> main.rs"));
        assert!(output_content.contains("fn main() { println!(\"Hello world\"); }"));

        assert!(!output_content.contains("==> main_test.rs"));
        assert!(!output_content.contains("fn test_main() { println!(\"Test hello world\"); }"));

        Ok(())
    }
}
