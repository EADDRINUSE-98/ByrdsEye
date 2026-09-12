use record::HashStatus;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
struct Size {
    old_size: Option<u64>,
    new_size: Option<u64>,
}
#[derive(Debug)]
struct Hash {
    old_hash: Option<[u8; 32]>,
    new_hash: Option<[u8; 32]>,
}

#[derive(Debug)]
struct Status {
    old_status: HashStatus,
    new_status: HashStatus,
}

#[derive(Debug)]
enum FileT {
    Dir,
    Symlink,
    File,
}

#[derive(Debug)]
enum FileS {
    Dir,
    Symlink {
        old_target: PathBuf,
        new_target: PathBuf,
    },
    File {
        size: Size,
        hash: Hash,
        status: Status,
    },
}

#[derive(Debug)]
enum FileC {
    FileTypeChange { old_type: FileT, new_type: FileT },
    FileStateChange(FileS),
}

#[derive(Debug)]
enum ChangeType {
    FileChanges(FileC),
    MTime {
        old_mtime: SystemTime,
        new_mtime: SystemTime,
    },
    Permissions {
        old_perm: u32,
        new_perm: u32,
    },
    Uid {
        old_uid: u32,
        new_uid: u32,
    },
    Gid {
        old_gid: u32,
        new_gid: u32,
    },
}

#[derive(Debug)]
struct DiffResult {
    path: PathBuf,
    diff: Vec<ChangeType>,
}
