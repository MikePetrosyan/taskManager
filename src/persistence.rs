use crate::Project;
use std::{fs, path::PathBuf};

const STATE_FILE: &str = "projects.json";

pub fn state_file_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(STATE_FILE);
    let _ = fs::create_dir_all(path.parent().unwrap());
    path
}

pub fn load_state() -> Vec<Project> {
    let path = state_file_path();
    fs::read_to_string(&path)
        .ok()
        .and_then(|j| serde_json::from_str(&j).ok())
        .unwrap_or_default()
}

pub fn save_state(projects: &[Project]) {
    if let Ok(json) = serde_json::to_string_pretty(projects) {
        let _ = fs::write(state_file_path(), json);
    }
}
