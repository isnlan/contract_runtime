use error::*;
use infer::archive::is_zip;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::{fs, io};
use zip::read::read_zipfile_from_stream;
use zip::ZipArchive;

pub fn unpack(v: &[u8], dist: Option<&str>) -> Result<()> {
    if is_zip(v) {
        return unpack_zip(v, dist);
    }

    Err(anyhow!("unsupported file type"))
}

pub fn unpack_zip(v: &[u8], dist: Option<&str>) -> Result<()> {
    let c = Cursor::new(v);
    let mut archive = ZipArchive::new(c)?;

    let mut archive_base = String::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if i == 0 {
            archive_base = String::from(outpath.clone().to_str().unwrap_or(""));
        }

        if let Some(root) = dist {
            let path = outpath.strip_prefix(&archive_base)?;
            outpath = Path::new(root).join(path);
        }

        {
            let comment = file.comment();
            if !comment.is_empty() {
                info!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            info!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            info!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
