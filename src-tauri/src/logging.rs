use std::fs;
use std::path::PathBuf;
use log::LevelFilter;
use env_logger::Builder;
use std::io::Write;

pub fn init_file_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create ~/.ClaudiaX directory if it doesn't exist
    let log_dir = get_log_directory()?;
    fs::create_dir_all(&log_dir)?;
    
    // Create backend log file path
    let backend_log_path = log_dir.join("backend.log");
    
    // Initialize file logger
    let target = Box::new(std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&backend_log_path)?);
    
    Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter_level(LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(buf, "[{}] [{}] [{}:{}] {}",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
    
    log::info!("ClaudiaX backend logging initialized at: {}", backend_log_path.display());
    
    Ok(())
}

pub fn get_log_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?;
    
    Ok(home_dir.join(".ClaudiaX"))
}

pub fn get_frontend_log_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let log_dir = get_log_directory()?;
    Ok(log_dir.join("frontend.log"))
}