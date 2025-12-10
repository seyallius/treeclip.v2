use crate::commands::run::args::RunArgs;
use std::path::Path;
use walkdir::WalkDir;

pub fn process_dir(run_args: &RunArgs) -> anyhow::Result<()> {
    validate_path_exists(&run_args.input_path)?;
    log_starting_path(&run_args.input_path);
    traverse_directory(
        &run_args.input_path,
        &run_args.exclude,
        run_args.skip_hidden,
        run_args.verbose,
    )?;
    println!("✅ Extraction complete");
    Ok(())
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

fn traverse_directory(
    root: &Path,
    exclude_patterns: &[String],
    skip_hidden: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let walker = WalkDir::new(root).into_iter().filter_entry(|entry| {
        let non_excluded_path = !should_exclude(entry.path(), exclude_patterns);
        let non_hidden_path = !skip_hidden || !is_hidden(entry);
        non_excluded_path && non_hidden_path
    });

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if verbose {
                println!("📄 {}", path.display());
            }
            // TODO: Process file content
        } else if path.is_dir() {
            if verbose {
                println!("📁 {}", path.display());
            }
        }
    }

    Ok(())
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
        let result = traverse_directory(temp_dir.path(), &[], false, false);
        assert!(result.is_ok());
    }
}