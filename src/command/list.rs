use crate::fs_util::{
    AGENT_DIR, CLAUDE_MD, SKILL_DIR, SKILL_MD, TYPE_AGENT, TYPE_CLAUDE_MD, TYPE_SKILL, agent_names,
    file_line_count, skill_names,
};
use std::env;
use std::path::Path;
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Information {
    types: String,
    file_name: String,
    line_count: usize,
}

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    let cwd =
        env::current_dir().map_err(|e| format!("current directory could not be read: {e}"))?;

    // Count the number of lines in CLAUDE.md
    let claude_md_path = cwd.join(CLAUDE_MD);
    let line_count = file_line_count(&claude_md_path)
        .map_err(|e| format!("{} could not be read: {e}", claude_md_path.display()))?;

    let mut rows = vec![Information {
        types: TYPE_CLAUDE_MD.into(),
        file_name: CLAUDE_MD.into(),
        line_count,
    }];

    // Count the number of lines in SKILL.md
    for d in skill_names(&cwd)? {
        let skills_md_path = cwd.join(SKILL_DIR).join(&d).join(SKILL_MD);
        let line_count = file_line_count(&skills_md_path)
            .map_err(|e| format!("{} could not be read: {e}", skills_md_path.display()))?;

        let row = Information {
            types: TYPE_SKILL.into(),
            file_name: d,
            line_count,
        };
        rows.push(row);
    }

    // Count the number of lines in Agents
    for d in agent_names(&cwd)? {
        let agents_md_path = cwd.join(AGENT_DIR).join(&d);

        let line_count = file_line_count(&agents_md_path)
            .map_err(|e| format!("{} could not be read: {e}", agents_md_path.display()))?;

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

    Ok(())
}
