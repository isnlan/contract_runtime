use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use error::*;

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

pub fn exists(path: impl Into<PathBuf>, sub: &str) -> bool {
    let p = path.into().join(sub);
    p.exists()
}

pub fn list_sub_dir(path: impl Into<PathBuf>) -> Result<Vec<String>> {
    let dir = path.into().read_dir()?;
    let list = dir.filter(|r|{
        match r {
            Ok(entry) => {
               entry.path().is_dir()
            }
            Err(_) => false
        }
    }).map(|dir|String::from(dir.unwrap().file_name().to_str().unwrap()))
        .collect();
    Ok(list)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use crate::path::{exists, list_sub_dir};
    use std::fs::File;

    #[test]
    fn it_works() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        assert!(!exists(path, "fs"));
        let fpath = path.join("1.txt");
        let f = File::create(fpath).unwrap();
        assert!(exists(path, "1.txt"));
    }

    #[test]
    fn test_list_sub_dir() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::create_dir_all(temp_dir.path().join("d1")).unwrap();
        std::fs::create_dir_all(temp_dir.path().join("d2")).unwrap();
        std::fs::create_dir_all(temp_dir.path().join("d3")).unwrap();
        let list = list_sub_dir(temp_dir.path()).unwrap();
        println!("{:?}", list);
    }
}
