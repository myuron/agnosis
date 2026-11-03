use crate::config;
use crate::fs_util::{
    AGENT_DIR, CLAUDE_MD, SKILL_DIR, SKILL_MD, agent_names, file_line_count, skill_names,
};
use std::env;

pub fn lint() -> Result<(), Box<dyn std::error::Error>> {
    let cwd =
        env::current_dir().map_err(|e| format!("current directory could not be read: {e}"))?;
    let config = config::load()?;
    let mut violations = 0;

    // Count the number of lines in CLAUDE.md
    let claude_md_path = cwd.join(CLAUDE_MD);
    let line_count = file_line_count(&claude_md_path)
        .map_err(|e| format!("{} could not be read: {e}", claude_md_path.display()))?;
    if !check(CLAUDE_MD, line_count, config.claude_md.max_lines) {
        violations += 1;
    }

    // Count the number of lines in SKILL.md
    for d in skill_names(&cwd)? {
        let skills_md_path = cwd.join(SKILL_DIR).join(&d).join(SKILL_MD);
        let line_count = file_line_count(&skills_md_path)
            .map_err(|e| format!("{} could not be read: {e}", skills_md_path.display()))?;
        if !check(&d, line_count, config.skills.max_lines) {
            violations += 1;
        }
    }

    // Count the number of lines in Agent
    for d in agent_names(&cwd)? {
        let agents_md_path = cwd.join(AGENT_DIR).join(&d);
        let line_count = file_line_count(&agents_md_path)
            .map_err(|e| format!("{} could not be read: {e}", agents_md_path.display()))?;
        if !check(&d, line_count, config.agents.max_lines) {
            violations += 1;
        }
    }

    if violations > 0 {
        return Err(format!("{violations} file(s) exceeded the line limit").into());
    }

    Ok(())
}

fn check(name: &str, line_count: usize, max_lines: usize) -> bool {
    if line_count <= max_lines {
        true
    } else {
        eprintln!("{name}: {line_count}/{max_lines} NG");
        false
    }
}
