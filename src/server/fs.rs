//! Converts between std::fs representations and 9p representations.

use nine::p2000::{self, *};
use std::fs::{self, *};
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

fn file_type(fs: &Metadata) -> p2000::FileType {
    use self::p2000::FileType as NineType;

    if fs.is_dir() {
        NineType::DIR
    } else if fs.is_file() {
        NineType::FILE
    } else {
        panic!("unknown file type")
    }
}

fn mode(meta: &Metadata) -> FileMode {
    let perms = meta.permissions();
    let mode = perms.mode()
}

fn qid(meta: &Metadata) -> Qid {
    Qid {
        file_type: file_type(meta),
        path: meta.st_ino(),
        version: 0
    }
}

fn stat(meta: &Metadata) -> Stat {
    Stat {
        type_: 0,
        dev: 0,
        qid: qid(meta),
        mode: mode(meta)
    }
    unimplemented!()
}