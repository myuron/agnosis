use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use tabled::settings::Style;
use tabled::{Table, Tabled};

const CLAUDE_MD: &str = "CLAUDE.md";

#[derive(Tabled)]
struct Information {
    file_name: String,
    line_count: usize,
}

fn main() {
    let cwd = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("current directory could not be read: {e}");
            std::process::exit(1);
        }
    };

    let claude_md_path = cwd.join(CLAUDE_MD);
    let line_count = match file_line_count(&claude_md_path) {
        Ok(lc) => lc,
        Err(e) => {
            eprintln!("{} could not be read: {e}", claude_md_path.display());
            std::process::exit(1);
        }
    };
    let rows = vec![Information {
        file_name: CLAUDE_MD.into(),
        line_count,
    }];

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
}
