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
    /// .aiignore files (alias for compatibility)
    Ai,
    /// .codexignore files (canonical)
    Codex,
}

impl Family {
    /// Get the filename for this family
    pub fn filename(&self) -> &'static str {
        match self {
            Family::Ai => ".aiignore",
            Family::Codex => ".codexignore",
        }
    }

    /// Get the environment variable prefix for this family
    pub fn env_prefix(&self) -> &'static str {
        match self {
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

/// Information about why a path matched or didn't match ignore rules
#[derive(Debug, Clone)]
pub struct ExplainStep {
    /// Whether the pattern matched
    pub matched: bool,
    /// The pattern that matched/didn't match
    pub pattern: String,
    /// Whether this was a negation pattern
    pub negated: bool,
    /// Line number in the ignore file
    pub line: usize,
    /// Source information
    pub source: IgnoreSource,
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
    pub fn matches(&self, rel_path: &Path, is_dir: bool) -> bool {
        // Find the most specific directory matcher for this path
        let path_absolute = self.root.join(rel_path);
        let mut best_matcher = None;
        let mut best_dir = PathBuf::new();

        for (dir, matcher) in &self.matchers {
            if dir.as_os_str().is_empty() || path_absolute.starts_with(dir) {
                if dir.components().count() > best_dir.components().count() {
                    best_matcher = Some(matcher);
                    best_dir = dir.clone();
                }
            }
        }

        if let Some(matcher) = best_matcher {
            match matcher.matched(rel_path, is_dir) {
                ignore::Match::None => false,
                ignore::Match::Ignore(_) => true,
                ignore::Match::Whitelist(_) => false,
            }
        } else {
            false
        }
    }

    /// Get detailed explanation of why a path matched or didn't match
    pub fn explain(&self, rel_path: &Path, is_dir: bool) -> Vec<ExplainStep> {
        // This is a simplified implementation - a full implementation would
        // need to parse each ignore file and track individual pattern matches
        let matched = self.matches(rel_path, is_dir);

        // For now, return a simple explanation
        if matched {
            vec![ExplainStep {
                matched: true,
                pattern: "**".to_string(), // Placeholder
                negated: false,
                line: 1,
                source: self
                    .sources
                    .first()
                    .cloned()
                    .unwrap_or_else(|| IgnoreSource {
                        file_path: PathBuf::from("unknown"),
                        family: Family::Codex,
                        scope: Scope::Directory,
                        dir: PathBuf::new(),
                        order: 0,
                    }),
            }]
        } else {
            vec![]
        }
    }

    /// Get all effective rules in precedence order
    pub fn get_effective_rules(&self) -> Vec<EffectiveRule> {
        // This would need to parse all ignore files and return their patterns
        // For now, return empty list
        vec![]
    }

    /// Get all sources used by this matcher
    pub fn sources(&self) -> &[IgnoreSource] {
        &self.sources
    }
}

/// An effective rule from an ignore file
#[derive(Debug, Clone)]
pub struct EffectiveRule {
    /// The pattern
    pub pattern: String,
    /// Source information
    pub source: IgnoreSource,
    /// Line number in the file
    pub line: usize,
}

/// Discover ignore files according to the hierarchy and precedence rules
pub struct IgnoreDiscovery {
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
        let ai_disabled = env::var("AI_IGNORE_DISABLE").is_ok_and(|v| v == "1");
        let codex_disabled = env::var("CODEX_IGNORE_DISABLE").is_ok_and(|v| v == "1");
        let all_disabled = env::var("CODEX_NO_IGNORE").is_ok_and(|v| v == "1");

        let mut additional_files = Vec::new();

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
            ai_disabled,
            codex_disabled,
            all_disabled,
            additional_files,
        }
    }

    /// Create a new IgnoreDiscovery with explicit family controls
    pub fn with_controls(ai_disabled: bool, codex_disabled: bool, all_disabled: bool) -> Self {
        let mut additional_files = Vec::new();

        // Handle AI_IGNORE_FILE environment variable
        if !ai_disabled && !all_disabled {
            if let Ok(files) = env::var("AI_IGNORE_FILE") {
                for file_path in files.split(',') {
                    let path = PathBuf::from(file_path.trim());
                    if path.is_absolute() {
                        additional_files.push((Family::Ai, path));
                    }
                }
            }
        }

        // Handle CODEX_IGNORE_FILE environment variable
        if !codex_disabled && !all_disabled {
            if let Ok(files) = env::var("CODEX_IGNORE_FILE") {
                for file_path in files.split(',') {
                    let path = PathBuf::from(file_path.trim());
                    if path.is_absolute() {
                        additional_files.push((Family::Codex, path));
                    }
                }
            }
        }

        Self {
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

        // 1. System global files
        if !self.ai_disabled {
            sources.extend(self.discover_system_global(Family::Ai, &mut order)?);
        }
        if !self.codex_disabled {
            sources.extend(self.discover_system_global(Family::Codex, &mut order)?);
        }

        // 2. User global files
        if !self.ai_disabled {
            sources.extend(self.discover_user_global(Family::Ai, &mut order)?);
        }
        if !self.codex_disabled {
            sources.extend(self.discover_user_global(Family::Codex, &mut order)?);
        }

        // 3. Project root files
        if !self.ai_disabled {
            if let Some(source) = self.discover_project_file(
                project_root,
                Family::Ai,
                Scope::ProjectRoot,
                &mut order,
            )? {
                sources.push(source);
            }
        }
        if !self.codex_disabled {
            if let Some(source) = self.discover_project_file(
                project_root,
                Family::Codex,
                Scope::ProjectRoot,
                &mut order,
            )? {
                sources.push(source);
            }
        }

        // 4. Additional files from environment/CLI
        for (family, path) in &self.additional_files {
            if (*family == Family::Ai && self.ai_disabled)
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

    /// Discover ignore files for a specific directory (used during traversal)
    pub fn discover_for_directory(
        &self,
        dir: &Path,
        _project_root: &Path,
        order: &mut usize,
    ) -> Result<Vec<IgnoreSource>> {
        if self.all_disabled {
            return Ok(vec![]);
        }

        let mut sources = Vec::new();

        // Add .aiignore then .codexignore for this directory
        if !self.ai_disabled {
            if let Some(source) =
                self.discover_project_file(dir, Family::Ai, Scope::Directory, order)?
            {
                sources.push(source);
            }
        }
        if !self.codex_disabled {
            if let Some(source) =
                self.discover_project_file(dir, Family::Codex, Scope::Directory, order)?
            {
                sources.push(source);
            }
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
}

/// Create an IgnoreMatcher for a project
pub fn create_ignore_matcher(project_root: &Path) -> Result<IgnoreMatcher> {
    let discovery = IgnoreDiscovery::new();
    let sources = discovery.discover(project_root)?;
    IgnoreMatcher::new(sources, project_root.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_family_filename() {
        assert_eq!(Family::Ai.filename(), ".aiignore");
        assert_eq!(Family::Codex.filename(), ".codexignore");
    }

    #[test]
    fn test_family_env_prefix() {
        assert_eq!(Family::Ai.env_prefix(), "AI");
        assert_eq!(Family::Codex.env_prefix(), "CODEX");
    }

    #[test]
    fn test_scope_ordering() {
        assert!(Scope::System < Scope::User);
        assert!(Scope::User < Scope::ProjectRoot);
        assert!(Scope::ProjectRoot < Scope::Directory);
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

        let matcher = create_ignore_matcher(root)?;

        // Test that .tmp files are ignored
        assert!(matcher.matches(Path::new("test.tmp"), false));
        assert!(!matcher.matches(Path::new("test.txt"), false));

        Ok(())
    }
}
