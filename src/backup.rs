use std::fs::{copy, create_dir, create_dir_all, read_dir, File};
use std::io::{Error, ErrorKind, Result, Write};
use std::path::Path;
use std::time::Duration;

use chrono::Local;
use fancy_duration::FancyDuration;

fn single_file(source: &Path, destination_dir: &Path) -> Result<u64> {
    let file_name = source.file_name().unwrap();
    if !destination_dir.exists() {
        create_dir(destination_dir)?;
    }
    let size = copy(source, destination_dir.join(file_name))?;
    Ok(size)
}

pub fn all(source_dir: &Path, destination_dir: &Path) -> Result<u64> {
    if !source_dir.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Path source in the config file is wrong!",
        ));
    }
    if !destination_dir.exists() {
        create_dir_all(destination_dir)?;
    }
    let mut size = 0;
    if source_dir.is_dir() {
        for entry in read_dir(source_dir)? {
            let sub_size;
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let file_name = path.file_name().unwrap();
                sub_size = all(&path, destination_dir.join(file_name).as_path())?;
            } else {
                sub_size = single_file(&path, destination_dir)?;
            }
            size += sub_size;
        }
    }
    Ok(size)
}

pub fn create_backup_summary(size: u64, cpu_time: Duration, destination_dir: &Path) -> Result<()> {
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(destination_dir.join("backup_summary.txt"))?;

    let cpu_time_string: String = FancyDuration(cpu_time).to_string();
    let date_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    write!(
        file,
        "[{}] Bytes written: {} B, CPU time: {}\n",
        date_time, size, cpu_time_string
    )?;

    Ok(())
}
