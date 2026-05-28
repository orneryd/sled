use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

use fs2::FileExt;

const DIRECTORY_LOCK_FILE: &str = ".sled-directory-lock";

pub(crate) fn sync_directory(path: &Path) -> io::Result<()> {
    #[cfg(windows)]
    {
        let _ = path;
        Ok(())
    }

    #[cfg(not(windows))]
    {
        std::fs::File::open(path).and_then(|file| file.sync_all())
    }
}

pub(crate) fn open_and_lock(path: &Path) -> io::Result<File> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path.join(DIRECTORY_LOCK_FILE))?;
    file.try_lock_exclusive()?;
    Ok(file)
}
