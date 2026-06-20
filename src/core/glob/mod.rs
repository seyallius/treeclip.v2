//! mod - Expands glob-pattern input arguments into concrete filesystem paths.

use crate::core::errors::PatternError;
use anyhow::Context;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Characters that mark a string as a glob pattern rather than a literal path.
const GLOB_METACHARACTERS: [char; 4] = ['*', '?', '[', '{'];

// -------------------------------------------- Public Functions --------------------------------------------

/// Returns `true` if the given input string contains glob metacharacters.
///
/// Used to decide whether an input argument should be treated as a literal
/// path (fast path, current behavior) or expanded against the filesystem.
///
/// # Examples
///
/// ```
/// use treeclip::core::glob::is_glob_pattern;
///
/// assert!(is_glob_pattern("object/*.go"));
/// assert!(is_glob_pattern("object*"));
/// assert!(!is_glob_pattern("src/main.rs"));
/// ```
pub fn is_glob_pattern(input: &str) -> bool {
    input.chars().any(|c| GLOB_METACHARACTERS.contains(&c))
}

/// Expands a single glob pattern into the list of concrete paths it matches.
///
/// Matching uses git-style glob semantics (the same engine that powers
/// `.gitignore` and this project's `--exclude` flag), so patterns like
/// `object/*`, `object*`, `object/*.go`, and `**/*.rs` behave the way a
/// developer would expect from `git` or `ripgrep` - shell-independent,
/// meaning they work identically whether or not the user's shell already
/// expanded them.
///
/// # Errors
///
/// Returns an error if:
/// - The pattern is not valid glob syntax
/// - The pattern matches zero paths on disk
pub fn expand_glob(pattern: &str) -> anyhow::Result<Vec<PathBuf>> {
    let base_dir = resolve_base_dir(pattern);

    let overrides = build_override_matcher(pattern, &base_dir)
        .with_context(|| format!("Failed to compile glob pattern: '{pattern}'"))?;

    let matches: Vec<PathBuf> = WalkBuilder::new(&base_dir)
        .hidden(false) // Let the run command's own --skip-hidden flag decide later
        .git_ignore(false) // Globs are explicit user intent, ignore files apply during traversal
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let relative = entry.path().strip_prefix(&base_dir).unwrap_or(entry.path());
            overrides
                .matched(relative, entry.path().is_dir())
                .is_whitelist()
        })
        .map(|entry| entry.path().to_path_buf())
        .collect();

    if matches.is_empty() {
        return Err(PatternError::GlobNoMatches {
            pattern: pattern.to_string(),
        }
        .into());
    }

    Ok(matches)
}

/// Expands a list of input strings, passing through literal (non-glob) paths
/// untouched and expanding any glob patterns into their matched paths.
///
/// Order is preserved as much as possible: each input contributes its
/// resulting path(s) at the position it appeared in.
///
/// # Errors
///
/// Returns an error if any glob pattern fails to compile or matches nothing.
pub fn expand_inputs(inputs: &[String]) -> anyhow::Result<Vec<PathBuf>> {
    let mut expanded = Vec::new();

    for input in inputs {
        if is_glob_pattern(input) {
            let matched = expand_glob(input)
                .with_context(|| format!("Failed to expand glob input: '{input}'"))?;
            expanded.extend(matched);
        } else {
            expanded.push(PathBuf::from(input));
        }
    }

    Ok(expanded)
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Resolves the directory to start walking from for a given glob pattern.
///
/// Takes everything before the first path component containing a glob
/// metacharacter, so `object/*.go` walks from `object/` instead of `.`,
/// keeping expansion fast and scoped on large projects.
fn resolve_base_dir(pattern: &str) -> PathBuf {
    let mut base = PathBuf::new();

    for component in Path::new(pattern).components() {
        let component_str = component.as_os_str().to_string_lossy();
        if is_glob_pattern(&component_str) {
            break;
        }
        base.push(component);
    }

    if base.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        base
    }
}

/// Builds a single-pattern override matcher rooted at `base_dir`.
///
/// The pattern is re-expressed relative to `base_dir` so the override
/// matcher (which matches relative paths) lines up with the walker rooted
/// at the same directory.
fn build_override_matcher(
    pattern: &str,
    base_dir: &Path,
) -> anyhow::Result<ignore::overrides::Override> {
    let relative_pattern = Path::new(pattern)
        .strip_prefix(base_dir)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| pattern.to_string());

    let mut builder = OverrideBuilder::new(base_dir);
    builder
        .add(&relative_pattern)
        .map_err(|e| PatternError::GlobBuildFailed {
            pattern: pattern.to_string(),
            source: e,
        })?;

    builder.build().map_err(|e| {
        PatternError::GlobBuildFailed {
            pattern: pattern.to_string(),
            source: e,
        }
        .into()
    })
}

#[cfg(test)]
mod glob_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_glob_pattern_detects_star() {
        assert!(is_glob_pattern("object/*"));
        assert!(is_glob_pattern("object*"));
        assert!(is_glob_pattern("object/*.go"));
    }

    #[test]
    fn test_is_glob_pattern_detects_question_and_brackets() {
        assert!(is_glob_pattern("file?.txt"));
        assert!(is_glob_pattern("file[0-9].txt"));
        assert!(is_glob_pattern("{src,test}/*.rs"));
    }

    #[test]
    fn test_is_glob_pattern_rejects_literal_paths() {
        assert!(!is_glob_pattern("src/main.rs"));
        assert!(!is_glob_pattern("."));
        assert!(!is_glob_pattern("./some/dir"));
    }

    #[test]
    fn test_resolve_base_dir_with_nested_glob() {
        assert_eq!(resolve_base_dir("object/*.go"), PathBuf::from("object"));
        assert_eq!(resolve_base_dir("src/core/*.rs"), PathBuf::from("src/core"));
    }

    #[test]
    fn test_resolve_base_dir_with_glob_at_root() {
        assert_eq!(resolve_base_dir("object*"), PathBuf::from("."));
        assert_eq!(resolve_base_dir("*.go"), PathBuf::from("."));
    }

    #[test]
    fn test_expand_glob_matches_extension_pattern() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let object_dir = temp_dir.path().join("object");
        fs::create_dir(&object_dir)?;
        fs::write(object_dir.join("a.go"), "package main")?;
        fs::write(object_dir.join("b.go"), "package main")?;
        fs::write(object_dir.join("c.txt"), "not go")?;

        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(temp_dir.path())?;

        let result = expand_glob("object/*.go");

        std::env::set_current_dir(original_dir)?;

        let matches = result?;
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().all(|p| p.extension().unwrap() == "go"));

        Ok(())
    }

    #[test]
    fn test_expand_glob_prefix_pattern() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        fs::create_dir(temp_dir.path().join("object_one"))?;
        fs::create_dir(temp_dir.path().join("object_two"))?;
        fs::create_dir(temp_dir.path().join("other"))?;

        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(temp_dir.path())?;

        let result = expand_glob("object*");

        std::env::set_current_dir(original_dir)?;

        let matches = result?;
        assert!(matches.iter().any(|p| p.ends_with("object_one")));
        assert!(matches.iter().any(|p| p.ends_with("object_two")));
        assert!(!matches.iter().any(|p| p.ends_with("other")));

        Ok(())
    }

    #[test]
    fn test_expand_glob_no_matches_errors() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;

        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(temp_dir.path())?;

        let result = expand_glob("nonexistent/*.rs");

        std::env::set_current_dir(original_dir)?;

        assert!(result.is_err());
        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("matched no files") || error_msg.contains("GlobNoMatches"));

        Ok(())
    }

    #[test]
    fn test_expand_inputs_mixes_literal_and_glob() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let object_dir = temp_dir.path().join("object");
        fs::create_dir(&object_dir)?;
        fs::write(object_dir.join("a.go"), "package main")?;
        fs::create_dir(temp_dir.path().join("src"))?;

        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(temp_dir.path())?;

        let inputs = vec!["src".to_string(), "object/*.go".to_string()];
        let result = expand_inputs(&inputs);

        std::env::set_current_dir(original_dir)?;

        let expanded = result?;
        assert!(expanded.iter().any(|p| p == &PathBuf::from("src")));
        assert!(expanded.iter().any(|p| p.ends_with("a.go")));

        Ok(())
    }

    #[test]
    fn test_expand_glob_recursive_doublestar() -> anyhow::Result<()> {
        // Git-style `**` should recurse through nested directories.
        let temp_dir = TempDir::new()?;
        let nested = temp_dir.path().join("src").join("core").join("traversal");
        fs::create_dir_all(&nested)?;
        fs::write(nested.join("walker.rs"), "// rust")?;
        fs::write(temp_dir.path().join("src").join("main.rs"), "// rust")?;
        fs::write(temp_dir.path().join("README.md"), "# readme")?;

        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(temp_dir.path())?;

        let result = expand_glob("src/**/*.rs");

        std::env::set_current_dir(original_dir)?;

        let matches = result?;
        assert!(matches.iter().any(|p| p.ends_with("walker.rs")));
        assert!(matches.iter().any(|p| p.ends_with("main.rs")));
        assert!(!matches.iter().any(|p| p.ends_with("README.md")));

        Ok(())
    }
}
