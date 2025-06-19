use std::io::{Read, Write};
use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json;
use timesman_type::{File, FileType, Pid, Tid};

pub(crate) struct FsFileStorage {
    base: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct FileMeta {
    name: String,
    ftype: FileType,
}

fn get_meta_file_path(pid: Pid) -> String {
    format!("{}.meta", pid)
}

impl FsFileStorage {
    pub fn new(base: PathBuf) -> Result<Self, String> {
        if !base.exists() {
            fs::create_dir(&base).map_err(|e| format!("{e}"))?;
        }

        Ok(Self { base })
    }

    pub fn save_file(
        &self,
        tid: Tid,
        pid: Pid,
        file: File,
    ) -> Result<(), String> {
        let dir = self.base.join(format!("{tid}"));

        if !dir.exists() {
            fs::create_dir(&dir).map_err(|e| format!("{e}"))?;
        }

        let data_file = dir.join(format!("{pid}"));
        fs::write(&data_file, &file.data).map_err(|e| format!("{e}"))?;

        let meta_file = dir.join(get_meta_file_path(pid));
        let meta = FileMeta {
            name: file.name,
            ftype: file.ftype,
        };
        let meta_file =
            fs::File::open(meta_file).map_err(|e| format!("{e}"))?;
        serde_json::to_writer(meta_file, &meta).map_err(|e| format!("{e}"))?;

        Ok(())
    }

    pub fn load_file(&self, tid: Tid, pid: Pid) -> Result<File, String> {
        let dir = self.base.join(format!("{tid}"));

        let meta_file = dir.join(get_meta_file_path(pid));
        let meta_file =
            fs::File::open(&meta_file).map_err(|e| format!("{e}"))?;

        let meta: FileMeta =
            serde_json::from_reader(&meta_file).map_err(|e| format!("{e}"))?;

        let data_file = dir.join(format!("{pid}"));
        let mut data = vec![];
        let mut data_file =
            fs::File::open(&data_file).map_err(|e| format!("{e}"))?;
        data_file
            .read_to_end(&mut data)
            .map_err(|e| format!("{e}"))?;

        Ok(File {
            name: meta.name,
            data,
            ftype: meta.ftype,
        })
    }
}
