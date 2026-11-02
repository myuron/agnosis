use clap::{Parser, Subcommand};
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use tabled::settings::Style;
use tabled::{Table, Tabled};

const TYPE_CLAUDE_MD: &str = "CLAUDE.md";
const CLAUDE_MD: &str = "CLAUDE.md";

const TYPE_SKILL: &str = "Skills";
const SKILL_DIR: &str = ".claude/skills/";
const SKILL_MD: &str = "SKILL.md";

const TYPE_AGENT: &str = "Agents";
const AGENT_DIR: &str = ".claude/agents";

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Output a list of files and the number of lines in each file
    List,
}

#[derive(Tabled)]
struct Information {
    types: String,
    file_name: String,
    line_count: usize,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::List => list(),
    }
}

fn list() {
    let cwd = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("current directory could not be read: {e}");
            std::process::exit(1);
        }
    };

    // Count the number of lines in CLAUDE.md
    let claude_md_path = cwd.join(CLAUDE_MD);
    let line_count = match file_line_count(&claude_md_path) {
        Ok(lc) => lc,
        Err(e) => {
            eprintln!("{} could not be read: {e}", claude_md_path.display());
            std::process::exit(1);
        }
    };
    let mut rows = vec![Information {
        types: TYPE_CLAUDE_MD.into(),
        file_name: CLAUDE_MD.into(),
        line_count,
    }];

    // Count the number of lines in SKILL.md
    let skills_dir_path = cwd.join(SKILL_DIR);
    let skills_dirs = match dir_list(&skills_dir_path) {
        Ok(dirs) => dirs,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => {
            eprintln!("{} could not be read: {e}", skills_dir_path.display());
            std::process::exit(1);
        }
    };
    for d in skills_dirs {
        let skills_md_path = skills_dir_path.join(&d).join(SKILL_MD);
        let line_count = match file_line_count(&skills_md_path) {
            Ok(lc) => lc,
            Err(e) => {
                eprintln!("{} could not be read: {e}", skills_md_path.display());
                std::process::exit(1);
            }
        };
        let row = Information {
            types: TYPE_SKILL.into(),
            file_name: d,
            line_count,
        };
        rows.push(row);
    }

    // Count the number of lines in Agents
    let agents_dir_path = cwd.join(AGENT_DIR);
    let agents_dirs = match file_list(&agents_dir_path) {
        Ok(files) => files,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => {
            eprintln!("{} could not be read: {e}", agents_dir_path.display());
            std::process::exit(1);
        }
    };
    for d in agents_dirs {
        let agents_md_path = agents_dir_path.join(&d);
        let line_count = match file_line_count(&agents_md_path) {
            Ok(lc) => lc,
            Err(e) => {
                eprintln!("{} could not be read: {e}", agents_md_path.display());
                std::process::exit(1);
            }
        };
        let agent_name = Path::new(&d)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&d)
            .to_string();
        let row = Information {
            types: TYPE_AGENT.into(),
            file_name: agent_name,
            line_count,
        };
        rows.push(row);
    }

    // Show table
    let mut table = Table::new(rows);
    table.with(Style::rounded());
    println!("{table}");
}

fn file_line_count(file_name: &Path) -> Result<usize, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_line_count_of_claudemd() {
        let claude_md_path = Path::new("tests/success/").join(CLAUDE_MD);
        assert_eq!(file_line_count(&claude_md_path).unwrap(), 42);
    }

    #[test]
    fn file_line_count_missing_file() {
        let claude_md_path = Path::new("tests/no-such-dir/").join(CLAUDE_MD);
        assert!(file_line_count(&claude_md_path).is_err());
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
        let skill_dir_path = Path::new("tests/success/").join(SKILL_DIR);
        let list_of_skills_dir = dir_list(&skill_dir_path);
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
        let agent_dir_path = Path::new("tests/success/").join(AGENT_DIR);
        let list_of_agents_files = file_list(&agent_dir_path);
        assert_eq!(list_of_agents_files.unwrap(), v);
    }
}
