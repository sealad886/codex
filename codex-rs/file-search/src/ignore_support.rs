//! Ignore functionality for .codexignore and .aiignore files.
//!
//! This module provides a hierarchical ignore system that supports both `.codexignore` (canonical)
//! and `.aiignore` (alias) files with gitignore-compatible semantics. The system implements
//! a clear precedence hierarchy: system < user < project-root < nested directories.

use anyhow::Context;
use anyhow::Result;
use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::path::PathBuf;

/// The family/type of ignore file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Family {
    /// .gitignore files (Git standard, lowest precedence)
    Git,
    /// .aiignore files (alias for compatibility)
    Ai,
    /// .codexignore files (canonical, highest precedence)
    Codex,
}

impl Family {
    /// Get the filename for this family
    pub fn filename(&self) -> &'static str {
        match self {
            Family::Git => ".gitignore",
            Family::Ai => ".aiignore",
            Family::Codex => ".codexignore",
        }
    }

    /// Get the environment variable prefix for this family
    pub fn env_prefix(&self) -> &'static str {
        match self {
            Family::Git => "GIT",
            Family::Ai => "AI",
            Family::Codex => "CODEX",
        }
    }
}

/// The scope/precedence level of an ignore source
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scope {
    /// System-wide ignore files (lowest precedence)
    System,
    /// User-global ignore files
    User,
    /// Project root ignore files
    ProjectRoot,
    /// Directory-level ignore files (highest precedence)
    Directory,
}

/// A source of ignore patterns with metadata about its origin and precedence
#[derive(Debug, Clone)]
pub struct IgnoreSource {
    /// Absolute path to the ignore file
    pub file_path: PathBuf,
    /// Family of the ignore file (ai or codex)
    pub family: Family,
    /// Scope/precedence level
    pub scope: Scope,
    /// Directory this file applies to (absolute path, empty for system/user)
    pub dir: PathBuf,
    /// Global ordering index for stable merges
    pub order: usize,
}

/// Core ignore matcher that handles hierarchical ignore rules
#[derive(Debug)]
pub struct IgnoreMatcher {
    /// All ignore sources in precedence order
    sources: Vec<IgnoreSource>,
    /// Compiled gitignore matchers for each directory
    matchers: HashMap<PathBuf, Gitignore>,
    /// Root directory for path normalization
    root: PathBuf,
}

impl IgnoreMatcher {
    /// Create a new IgnoreMatcher from the given sources
    pub fn new(sources: Vec<IgnoreSource>, root: PathBuf) -> Result<Self> {
        let mut matcher = Self {
            sources,
            matchers: HashMap::new(),
            root,
        };
        matcher.build_matchers()?;
        Ok(matcher)
    }

    /// Build gitignore matchers for each directory scope
    fn build_matchers(&mut self) -> Result<()> {
        // Group sources by directory
        let mut by_dir: HashMap<PathBuf, Vec<&IgnoreSource>> = HashMap::new();

        for source in &self.sources {
            by_dir.entry(source.dir.clone()).or_default().push(source);
        }

        // Build a matcher for each directory
        for (dir, dir_sources) in by_dir {
            let mut builder = GitignoreBuilder::new(&self.root);

            // Sort sources by order to ensure stable precedence
            let mut sorted_sources = dir_sources;
            sorted_sources.sort_by_key(|s| s.order);

            // Add each ignore file to the builder
            for source in sorted_sources {
                if source.file_path.exists() {
                    builder.add(&source.file_path);
                }
            }

            let gitignore = builder
                .build()
                .context("Failed to build gitignore matcher")?;
            self.matchers.insert(dir, gitignore);
        }

        Ok(())
    }

    /// Check if a path should be ignored
    ///
    /// Returns true if the path should be ignored, false if it should be included.
    ///
    /// This method evaluates all applicable ignore scopes in precedence order,
    /// allowing later (more specific) rules to override earlier (more general) ones.
    pub fn matches(&self, rel_path: &Path, is_dir: bool) -> bool {
        let path_absolute = self.root.join(rel_path);
        let mut final_result = false;

        // Collect all applicable matchers with their directory depth for sorting
        let mut applicable_matchers: Vec<_> = self
            .matchers
            .iter()
            .filter(|(dir, _)| {
                // Empty dir means global (system/user), always applies
                dir.as_os_str().is_empty() || path_absolute.starts_with(dir)
            })
            .collect();

        // Sort by directory depth (global first, then increasingly specific directories)
        applicable_matchers.sort_by_key(|(dir, _)| dir.components().count());

        // Evaluate each applicable matcher in precedence order
        for (_, matcher) in applicable_matchers {
            match matcher.matched(rel_path, is_dir) {
                ignore::Match::None => {
                    // No rule matched, keep current result
                }
                ignore::Match::Ignore(_) => {
                    // Rule says to ignore, update result
                    final_result = true;
                }
                ignore::Match::Whitelist(_) => {
                    // Rule says to include (whitelist), update result
                    final_result = false;
                }
            }
        }

        final_result
    }

    /// Get all sources used by this matcher
    pub fn sources(&self) -> &[IgnoreSource] {
        &self.sources
    }
}

/// Discover ignore files according to the hierarchy and precedence rules
pub struct IgnoreDiscovery {
    /// Whether Git family files are disabled
    git_disabled: bool,
    /// Whether AI family files are disabled
    ai_disabled: bool,
    /// Whether Codex family files are disabled
    codex_disabled: bool,
    /// Whether all ignoring is disabled
    all_disabled: bool,
    /// Additional ignore files from environment/CLI
    additional_files: Vec<(Family, PathBuf)>,
}

impl Default for IgnoreDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl IgnoreDiscovery {
    /// Create a new IgnoreDiscovery with environment variable settings
    pub fn new() -> Self {
        let git_disabled = env::var("GIT_IGNORE_DISABLE").is_ok_and(|v| v == "1");
        let ai_disabled = env::var("AI_IGNORE_DISABLE").is_ok_and(|v| v == "1");
        let codex_disabled = env::var("CODEX_IGNORE_DISABLE").is_ok_and(|v| v == "1");
        let all_disabled = env::var("CODEX_NO_IGNORE").is_ok_and(|v| v == "1");

        let mut additional_files = Vec::new();

        // Handle GIT_IGNORE_FILE environment variable
        if let Ok(files) = env::var("GIT_IGNORE_FILE") {
            for file_path in files.split(',') {
                let path = PathBuf::from(file_path.trim());
                if path.is_absolute() {
                    additional_files.push((Family::Git, path));
                }
            }
        }

        // Handle AI_IGNORE_FILE environment variable
        if let Ok(files) = env::var("AI_IGNORE_FILE") {
            for file_path in files.split(',') {
                let path = PathBuf::from(file_path.trim());
                if path.is_absolute() {
                    additional_files.push((Family::Ai, path));
                }
            }
        }

        // Handle CODEX_IGNORE_FILE environment variable
        if let Ok(files) = env::var("CODEX_IGNORE_FILE") {
            for file_path in files.split(',') {
                let path = PathBuf::from(file_path.trim());
                if path.is_absolute() {
                    additional_files.push((Family::Codex, path));
                }
            }
        }

        Self {
            git_disabled,
            ai_disabled,
            codex_disabled,
            all_disabled,
            additional_files,
        }
    }

    /// Create a new IgnoreDiscovery with explicit family controls
    pub fn with_controls(
        git_disabled: bool,
        ai_disabled: bool,
        codex_disabled: bool,
        all_disabled: bool,
    ) -> Self {
        let mut additional_files = Vec::new();

        // Handle environment variable files
        Self::add_env_files(&mut additional_files, Family::Git, "GIT_IGNORE_FILE");
        Self::add_env_files(&mut additional_files, Family::Ai, "AI_IGNORE_FILE");
        Self::add_env_files(&mut additional_files, Family::Codex, "CODEX_IGNORE_FILE");

        Self {
            git_disabled,
            ai_disabled,
            codex_disabled,
            all_disabled,
            additional_files,
        }
    }

    /// Discover all ignore sources for a project
    pub fn discover(&self, project_root: &Path) -> Result<Vec<IgnoreSource>> {
        if self.all_disabled {
            return Ok(vec![]);
        }

        let mut sources = Vec::new();
        let mut order = 0;

        // 1. System global files (in family precedence order)
        if !self.git_disabled {
            sources.extend(self.discover_system_global(Family::Git, &mut order)?);
        }
        if !self.ai_disabled {
            sources.extend(self.discover_system_global(Family::Ai, &mut order)?);
        }
        if !self.codex_disabled {
            sources.extend(self.discover_system_global(Family::Codex, &mut order)?);
        }

        // 2. User global files (in family precedence order)
        if !self.git_disabled {
            sources.extend(self.discover_user_global(Family::Git, &mut order)?);
        }
        if !self.ai_disabled {
            sources.extend(self.discover_user_global(Family::Ai, &mut order)?);
        }
        if !self.codex_disabled {
            sources.extend(self.discover_user_global(Family::Codex, &mut order)?);
        }

        // 3. Project root files (in family precedence order: .codexignore > .aiignore > .gitignore)
        if !self.git_disabled
            && let Some(source) = self.discover_project_file(
                project_root,
                Family::Git,
                Scope::ProjectRoot,
                &mut order,
            )?
        {
            sources.push(source);
        }
        if !self.ai_disabled
            && let Some(source) = self.discover_project_file(
                project_root,
                Family::Ai,
                Scope::ProjectRoot,
                &mut order,
            )?
        {
            sources.push(source);
        }
        if !self.codex_disabled
            && let Some(source) = self.discover_project_file(
                project_root,
                Family::Codex,
                Scope::ProjectRoot,
                &mut order,
            )?
        {
            sources.push(source);
        }

        // 4. Discover nested directory ignore files
        sources.extend(self.discover_nested_directories(project_root, &mut order)?);

        // 5. Additional files from environment/CLI
        for (family, path) in &self.additional_files {
            if (*family == Family::Git && self.git_disabled)
                || (*family == Family::Ai && self.ai_disabled)
                || (*family == Family::Codex && self.codex_disabled)
            {
                continue;
            }

            sources.push(IgnoreSource {
                file_path: path.clone(),
                family: *family,
                scope: Scope::ProjectRoot, // Treat as project-level
                dir: project_root.to_path_buf(),
                order,
            });
            order += 1;
        }

        Ok(sources)
    }

    fn discover_system_global(
        &self,
        family: Family,
        order: &mut usize,
    ) -> Result<Vec<IgnoreSource>> {
        let mut sources = Vec::new();
        let paths = self.get_system_global_paths(family);

        for path in paths {
            if path.exists() {
                sources.push(IgnoreSource {
                    file_path: path,
                    family,
                    scope: Scope::System,
                    dir: PathBuf::new(), // Empty for global
                    order: *order,
                });
                *order += 1;
            }
        }

        Ok(sources)
    }

    fn discover_user_global(&self, family: Family, order: &mut usize) -> Result<Vec<IgnoreSource>> {
        let mut sources = Vec::new();
        let paths = self.get_user_global_paths(family);

        for path in paths {
            if path.exists() {
                sources.push(IgnoreSource {
                    file_path: path,
                    family,
                    scope: Scope::User,
                    dir: PathBuf::new(), // Empty for global
                    order: *order,
                });
                *order += 1;
            }
        }

        Ok(sources)
    }

    fn discover_project_file(
        &self,
        dir: &Path,
        family: Family,
        scope: Scope,
        order: &mut usize,
    ) -> Result<Option<IgnoreSource>> {
        let path = dir.join(family.filename());
        if path.exists() {
            let source = IgnoreSource {
                file_path: path,
                family,
                scope,
                dir: dir.to_path_buf(),
                order: *order,
            };
            *order += 1;
            Ok(Some(source))
        } else {
            Ok(None)
        }
    }

    fn get_system_global_paths(&self, family: Family) -> Vec<PathBuf> {
        let prefix = family.env_prefix().to_lowercase();

        #[cfg(unix)]
        {
            vec![PathBuf::from(format!("/etc/{}/ignore", prefix))]
        }

        #[cfg(windows)]
        {
            if let Ok(program_data) = env::var("ProgramData") {
                vec![PathBuf::from(program_data).join(prefix).join("ignore")]
            } else {
                vec![]
            }
        }
    }

    fn get_user_global_paths(&self, family: Family) -> Vec<PathBuf> {
        let prefix = family.env_prefix().to_lowercase();

        #[cfg(unix)]
        {
            let config_home = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
                env::var("HOME")
                    .map(|h| format!("{}/.config", h))
                    .unwrap_or_default()
            });

            if !config_home.is_empty() {
                vec![PathBuf::from(config_home).join(prefix).join("ignore")]
            } else {
                vec![]
            }
        }

        #[cfg(windows)]
        {
            if let Ok(app_data) = env::var("AppData") {
                vec![PathBuf::from(app_data).join(prefix).join("ignore")]
            } else {
                vec![]
            }
        }
    }

    /// Discover ignore files in nested directories by walking the project tree
    fn discover_nested_directories(
        &self,
        project_root: &Path,
        order: &mut usize,
    ) -> Result<Vec<IgnoreSource>> {
        let mut sources = Vec::new();

        // Walk the directory tree to find ignore files
        // Use the ignore crate's WalkBuilder for efficient traversal
        let walker = ignore::WalkBuilder::new(project_root)
            .hidden(false) // Don't skip hidden files since ignore files might be in hidden dirs
            .ignore(false) // Don't use any existing ignore files for this discovery
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false)
            .build();

        for entry_result in walker {
            let entry = match entry_result {
                Ok(e) => e,
                Err(_) => continue, // Skip entries we can't read
            };

            // Only process directories
            let Some(file_type) = entry.file_type() else {
                continue;
            };
            if !file_type.is_dir() {
                continue;
            }

            let dir_path = entry.path();

            // Skip the project root since we already handled it
            if dir_path == project_root {
                continue;
            }

            // Look for ignore files in this directory
            for &family in &[Family::Git, Family::Ai, Family::Codex] {
                // Check if this family is disabled
                if (family == Family::Git && self.git_disabled)
                    || (family == Family::Ai && self.ai_disabled)
                    || (family == Family::Codex && self.codex_disabled)
                {
                    continue;
                }

                let ignore_file = dir_path.join(family.filename());
                if ignore_file.exists() {
                    sources.push(IgnoreSource {
                        file_path: ignore_file,
                        family,
                        scope: Scope::Directory,
                        dir: dir_path.to_path_buf(),
                        order: *order,
                    });
                    *order += 1;
                }
            }
        }

        Ok(sources)
    }

    /// Helper method to add environment variable sources for a family
    fn add_env_files(additional_files: &mut Vec<(Family, PathBuf)>, family: Family, env_var: &str) {
        if let Ok(files) = env::var(env_var) {
            for file_path in files.split(',') {
                let path = PathBuf::from(file_path.trim());
                if path.is_absolute() {
                    additional_files.push((family, path));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_family_filename() {
        assert_eq!(Family::Git.filename(), ".gitignore");
        assert_eq!(Family::Ai.filename(), ".aiignore");
        assert_eq!(Family::Codex.filename(), ".codexignore");
    }

    #[test]
    fn test_family_env_prefix() {
        assert_eq!(Family::Git.env_prefix(), "GIT");
        assert_eq!(Family::Ai.env_prefix(), "AI");
        assert_eq!(Family::Codex.env_prefix(), "CODEX");
    }

    #[test]
    fn test_scope_ordering() {
        assert!(Scope::System < Scope::User);
        assert!(Scope::User < Scope::ProjectRoot);
    }

    #[test]
    fn test_discover_project_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Create test ignore files
        fs::write(root.join(".aiignore"), "*.tmp\n")?;
        fs::write(root.join(".codexignore"), "*.log\n")?;

        let discovery = IgnoreDiscovery::new();
        let sources = discovery.discover(root)?;

        // Should find both files, with proper ordering (ai first, then codex)
        let project_sources: Vec<_> = sources
            .iter()
            .filter(|s| s.scope == Scope::ProjectRoot)
            .collect();

        assert_eq!(project_sources.len(), 2);

        // AI should come before Codex in the ordering
        let ai_source = project_sources
            .iter()
            .find(|s| s.family == Family::Ai)
            .unwrap();
        let codex_source = project_sources
            .iter()
            .find(|s| s.family == Family::Codex)
            .unwrap();
        assert!(ai_source.order < codex_source.order);

        Ok(())
    }

    #[test]
    fn test_ignore_matcher_basic() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Create a simple .codexignore file
        fs::write(root.join(".codexignore"), "*.tmp\n")?;

        let discovery = IgnoreDiscovery::new();
        let sources = discovery.discover(root)?;
        let matcher = IgnoreMatcher::new(sources, root.to_path_buf())?;

        // Test that .tmp files are ignored
        assert!(matcher.matches(Path::new("test.tmp"), false));
        assert!(!matcher.matches(Path::new("test.txt"), false));

        Ok(())
    }
}
