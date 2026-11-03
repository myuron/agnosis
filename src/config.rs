use serde::Deserialize;
use std::{env, fs};

pub const CONFIG_FILE: &str = "agnosis.toml";
pub const DEFAULT_CONFIG_FILE: &str = include_str!("../assets/agnosis.toml");

#[derive(Deserialize)]
pub struct Config {
    pub claude_md: ClaudeMd,
    pub skills: Skills,
    pub agents: Agents,
}

#[derive(Deserialize)]
pub struct ClaudeMd {
    pub max_lines: usize,
}

#[derive(Deserialize)]
pub struct Skills {
    pub max_lines: usize,
}

#[derive(Deserialize)]
pub struct Agents {
    pub max_lines: usize,
}

pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    let cwd =
        env::current_dir().map_err(|e| format!("current directory could not be read: {e}"))?;

    let config_file_path = cwd.join(CONFIG_FILE);

    let config_file = fs::read_to_string(&config_file_path).map_err(|e| {
        format!(
            "{} could not be read: {e}\nrun `agnosis init` to create it",
            config_file_path.display()
        )
    })?;

    let config = toml::from_str(&config_file)
        .map_err(|e| format!("{} could not be parsed: {e}", config_file_path.display()))?;

    Ok(config)
}
