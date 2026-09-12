use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
pub enum HashStatus {
    Verified,
    Unstable,
    PermissionDenied,
    Error(String),
}

#[derive(Debug)]
enum FileType {
    Dir,
    Symlink {
        target: PathBuf,
    },
    File {
        hash: Option<[u8; 32]>,
        status: HashStatus,
        size: u64,
    },
}

#[derive(Debug)]
struct FileRecord {
    path: PathBuf,
    mtime: SystemTime,
    permission: u32,
    uid: u32,
    gid: u32,
    filetype: FileType,
}
