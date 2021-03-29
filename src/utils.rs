use anyhow::Result;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub fn glob_file_path<P: AsRef<Path>>(path: P, extension: &str) -> Result<Vec<String>> {
    let dir = fs::read_dir(path)?;

    let v = dir
        .filter(|r| match r {
            Ok(entry) => {
                let path = entry.path();
                path.extension() == Some(OsStr::new(extension))
            }
            Err(_err) => false,
        })
        .map(|dir| String::from(dir.unwrap().path().to_str().unwrap()))
        .collect::<Vec<String>>();

    Ok(v)
}
