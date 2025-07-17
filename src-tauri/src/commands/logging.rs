use tauri::command;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn get_log_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?;
    
    Ok(home_dir.join(".ClaudiaX"))
}

fn get_frontend_log_path_internal() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let log_dir = get_log_directory()?;
    std::fs::create_dir_all(&log_dir)?;
    Ok(log_dir.join("frontend.log"))
}

#[command]
pub fn get_frontend_log_path() -> Result<String, String> {
    get_frontend_log_path_internal()
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[command]
pub fn write_frontend_log(level: String, message: String, timestamp: String) -> Result<(), String> {
    let log_path = get_frontend_log_path_internal()
        .map_err(|e| e.to_string())?;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| e.to_string())?;
    
    writeln!(file, "[{}] [{}] {}", timestamp, level, message)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}