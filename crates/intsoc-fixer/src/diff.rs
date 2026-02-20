// SPDX-License-Identifier: PMPL-1.0-or-later

//! Unified diff generation using the `similar` crate.

use similar::TextDiff;

/// Generate a unified diff between two strings.
#[must_use]
pub fn unified_diff(original: &str, modified: &str, filename: &str) -> String {
    let diff = TextDiff::from_lines(original, modified);

    diff.unified_diff()
        .header(&format!("a/{filename}"), &format!("b/{filename}"))
        .to_string()
}

/// Generate a colored inline diff for terminal display.
#[must_use]
pub fn inline_diff(original: &str, modified: &str) -> String {
    let diff = TextDiff::from_lines(original, modified);
    let mut output = String::new();

    for change in diff.iter_all_changes() {
        let (sign, color_start, color_end) = match change.tag() {
            similar::ChangeTag::Delete => ("-", "\x1b[31m", "\x1b[0m"),
            similar::ChangeTag::Insert => ("+", "\x1b[32m", "\x1b[0m"),
            similar::ChangeTag::Equal => (" ", "", ""),
        };
        output.push_str(&format!("{color_start}{sign}{}{color_end}", change.value()));
    }

    output
}

/// Count the number of changes between two strings.
#[must_use]
pub fn change_count(original: &str, modified: &str) -> ChangeStats {
    let diff = TextDiff::from_lines(original, modified);
    let mut insertions = 0usize;
    let mut deletions = 0usize;

    for change in diff.iter_all_changes() {
        match change.tag() {
            similar::ChangeTag::Delete => deletions += 1,
            similar::ChangeTag::Insert => insertions += 1,
            similar::ChangeTag::Equal => {}
        }
    }

    ChangeStats {
        insertions,
        deletions,
    }
}

/// Statistics about changes between two texts.
#[derive(Debug, Clone)]
pub struct ChangeStats {
    pub insertions: usize,
    pub deletions: usize,
}

impl ChangeStats {
    /// Total number of changed lines.
    #[must_use]
    pub fn total(&self) -> usize {
        self.insertions + self.deletions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_diff() {
        let original = "line 1\nline 2\nline 3\n";
        let modified = "line 1\nline 2 changed\nline 3\n";
        let diff = unified_diff(original, modified, "test.xml");
        assert!(diff.contains("--- a/test.xml"));
        assert!(diff.contains("+++ b/test.xml"));
    }

    #[test]
    fn test_change_count() {
        let original = "a\nb\nc\n";
        let modified = "a\nB\nc\n";
        let stats = change_count(original, modified);
        assert_eq!(stats.insertions, 1);
        assert_eq!(stats.deletions, 1);
    }
}
