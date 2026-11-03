use std::fs::{self, File};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

pub const TYPE_CLAUDE_MD: &str = "CLAUDE.md";
pub const CLAUDE_MD: &str = "CLAUDE.md";

pub const TYPE_SKILL: &str = "Skills";
pub const SKILL_DIR: &str = ".claude/skills";
pub const SKILL_MD: &str = "SKILL.md";

pub const TYPE_AGENT: &str = "Agents";
pub const AGENT_DIR: &str = ".claude/agents";

pub fn file_line_count(file_name: &Path) -> Result<usize, Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    // Count the number of lines
    let mut count = 0;
    for line in reader.lines() {
        line?;
        count += 1;
    }
    Ok(count)
}

fn dir_list(path: &Path) -> Result<Vec<String>, Error> {
    let mut dirs = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir()
            && let Some(name) = entry.file_name().to_str()
        {
            dirs.push(name.to_string());
        }
    }

    dirs.sort();
    Ok(dirs)
}

fn file_list(path: &Path) -> Result<Vec<String>, Error> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file()
            && let Some(name) = entry.file_name().to_str()
        {
            files.push(name.to_string());
        }
    }

    files.sort();
    Ok(files)
}

pub fn skill_names(root: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let skills_dir_path = root.join(SKILL_DIR);
    match dir_list(&skills_dir_path) {
        Ok(dirs) => Ok(dirs),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(format!("{} could not be read: {e}", skills_dir_path.display()).into()),
    }
}

pub fn agent_names(root: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let agents_dir_path = root.join(AGENT_DIR);
    match file_list(&agents_dir_path) {
        Ok(files) => Ok(files),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(format!("{} could not be read: {e}", agents_dir_path.display()).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_line_count_of_claudemd() {
        let claude_md_path = Path::new("tests/success/CLAUDE.md");
        assert_eq!(file_line_count(claude_md_path).unwrap(), 42);
    }

    #[test]
    fn file_line_count_missing_file() {
        let claude_md_path = Path::new("tests/no-such-dir/CLAUDE.md");
        assert!(file_line_count(claude_md_path).is_err());
    }

    #[test]
    fn list_of_skills_dir() {
        let v: Vec<&str> = vec![
            "api-doc-generator",
            "changelog-writer",
            "commit-message",
            "pr-review",
            "rust-test-runner",
        ];
        let skill_dir_path = Path::new("tests/success/.claude/skills/");
        let list_of_skills_dir = dir_list(skill_dir_path);
        assert_eq!(list_of_skills_dir.unwrap(), v);
    }

    #[test]
    fn list_of_agents_file() {
        let v: Vec<&str> = vec![
            "code-reviewer.md",
            "debugger.md",
            "doc-writer.md",
            "security-auditor.md",
            "test-runner.md",
        ];
        let agent_dir_path = Path::new("tests/success/.claude/agents");
        let list_of_agents_files = file_list(agent_dir_path);
        assert_eq!(list_of_agents_files.unwrap(), v);
    }
}
