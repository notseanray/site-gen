use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use ignore::{WalkState::Continue, WalkBuilder};
use notify::DebouncedEvent;
use ignore::DirEntry;
use std::thread;
use rsass::{compile_scss_path, output};

#[derive(Clone)]
pub struct Sml {
    hashset: HashMap<PathBuf, u64>,
    new_hashset: HashMap<PathBuf, u64>,
    global_dir: PathBuf,
    data_in: PathBuf,
    data_out: PathBuf,
    static_content: PathBuf
}

impl Sml {
    pub fn new(dir_in: Option<&str>, dir_global: Option<&str>, dir_out: Option<&str>, dir_static: Option<&str>) -> Self {
        let global_dir = Sml::inspect_dir(dir_global, "./");
        let data_in = Sml::inspect_dir(dir_global, "./content");
        let data_out = Sml::inspect_dir(dir_global, "./build");
        let static_content = Sml::inspect_dir(dir_global, "./static");
        Self {
            hashset: HashMap::new(),
            new_hashset: HashMap::new(),
            global_dir,
            data_in,
            data_out,
            static_content
        }
    }

    pub fn update(&mut self, threads: usize) {
        /*
        let mut new_hashes = HashMap::new();
        // check if is dir
            new_hashes.insert(
                file.to_owned(), 
                hash(fs::read_to_string(file)
                    .expect("failed to rehash data")
                    .as_bytes()
                )
            );
            */
        let (tx, rx) = crossbeam_channel::bounded::<DirEntry>(100);
        let stdout_thread = thread::spawn(move || {
            for dent in rx {
                let current_path = PathBuf::from(dent.path());
                if !current_path.exists() {
                    continue;
                }
                Sml::handle_file_type(dent);
            }
        });
        let walker = WalkBuilder::new(&self.data_in).threads(threads).build_parallel();
        walker.run(|| {
            let tx = tx.clone();
            Box::new(move |res| {
                if let Ok(v) = res {
                    let _ = tx.send(v);
                }
                Continue
            })
        });
        drop(tx);
        stdout_thread.join().unwrap();
        //self.update_hashset(); 
    }

    fn handle_file_type(entry: DirEntry) {
        let path = entry.into_path();
        match path.extension().unwrap().to_string_lossy().to_string().as_str() {
            "css" => Sml::handle_css(path),
            _ => {}
        }
    }

    fn inspect_dir(path: Option<&str>, default: &str) -> PathBuf {
        let entry = PathBuf::from_str(match path {
            Some(v) => v,
            None => default
        });
        match entry {
            Ok(v) => v,
            Err(_) => {
                panic!("failed to create path buf to {:?}", entry)
            }
        }
    }

    fn handle_css(file: PathBuf) {
        let format = output::Format {
            style: output::Style::Compressed,
            .. Default::default()
        };
        let css = compile_scss_path(&file, format).unwrap(); 
    }

    fn update_hashset(&mut self, file: PathBuf) {
    }
}

type Ev = DebouncedEvent;

pub fn event_handler(ev: &DebouncedEvent) -> bool {
    match ev {
        Ev::Write(_) => true,
        Ev::Create(_) => true,
        Ev::Remove(_) => true,
        _ => false
    }
}
