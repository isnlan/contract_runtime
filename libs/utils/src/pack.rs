use error::*;
use infer::archive::is_zip;
use std::{fs, io};
use zip::read::read_zipfile_from_stream;
use std::fs::File;

pub trait Unpack {
    fn unpack(&self, file: &fs::File) -> Result<()>;
}

pub struct ZipUnpack;

impl Unpack for ZipUnpack {
    fn unpack(&self, file: &File) -> Result<()> {
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

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
                io::copy(&mut file, &mut outfile)?;
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
}


