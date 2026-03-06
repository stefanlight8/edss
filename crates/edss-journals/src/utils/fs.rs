use std::{fs, io::Error, path::PathBuf};

pub fn get_journals(dir: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut journals: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().map(|s| s == "log").unwrap_or(false))
        .collect();

    journals.sort_by(|a, b| b.as_os_str().cmp(a.as_os_str()));

    Ok(journals)
}
