use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use crate::config::Config;
use egui;
use wasmtime::{Caller, Engine, Linker, Module, Store};

pub struct Plugin {
    mods: Vec<(String, Module)>,
}

impl Plugin {
    pub fn setup(config: &Config) -> Result<Plugin, String> {
        let engine = Engine::default();

        let mods = Self::load_modules(&engine, &config.config_path);
        let mut linker = Linker::new(&engine);
        linker
            .func_wrap("host", "info", |caller: Caller<'_, u32>, param: i32| {
                info!(format!("logging from wasm {}", param));
            })
            .unwrap();

        let mut store: Store<u32> = Store::new(&engine, 4);
        let instance = linker.instantiate(&mut store, &mods[0].1).unwrap();

        let hello = instance
            .get_typed_func::<(), ()>(&mut store, "init")
            .unwrap();

        hello.call(&mut store, ()).unwrap();

        Ok(Self { mods })
    }

    fn load_modules(engine: &Engine, path: &PathBuf) -> Vec<(String, Module)> {
        let files = Self::load_module_files(path.clone()).unwrap();

        let mut mods = vec![];

        for f in files {
            let mut buf = vec![];
            let name = f.0;
            let mut file = f.1;

            file.read_to_end(&mut buf).unwrap();

            let module = Module::from_binary(engine, &mut buf).unwrap();

            mods.push((name, module));
        }

        // sample module
        let wat = r#"
        (module
            (import "host" "info" (func $host_info (param i32)))

            (func (export "init")
                i32.const 3
                call $host_info)

            (func (export "update_bar")
            )
        )"#;
        let module = Module::new(&engine, wat).unwrap();
        mods.push(("sample".to_string(), module));

        mods
    }

    fn load_module_files(
        mut dir: PathBuf,
    ) -> Result<Vec<(String, File)>, String> {
        dir.push("plugins");

        if !dir.exists() {
            fs::create_dir(&dir).unwrap();
        }

        let entries = dir.read_dir().unwrap();

        let mut files = vec![];

        for entry in entries.into_iter() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                debug!("Found a directory. Ignored ");
                continue;
            }

            let file = File::open(path.clone()).unwrap();

            let name: String =
                path.file_stem().unwrap().to_string_lossy().to_string();

            files.push((name, file))
        }

        Ok(files)
    }

    pub fn update_bar(&mut self, ui: &mut egui::Ui) {
        for (name, p) in &self.mods {}
    }
}
