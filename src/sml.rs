use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;
use ignore::{WalkState::Continue, WalkBuilder};
use notify::DebouncedEvent;
use ignore::DirEntry;
use std::thread;
use rsass::{compile_scss_path, output};
use std::io::Result;

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
                let result = Sml::handle_file_type(dent);
                if let Ok(_) = result {
                    continue;
                };
                println!("error: {:#?}", result);
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

    fn handle_file_type(entry: DirEntry) -> Result<()> {
        let path = entry.into_path();
        let extension = match path.extension() {
            Some(v) => v,
            None => return Ok(())
        };
        match extension.to_string_lossy().to_string().as_str() {
            "css" | "scss" => Sml::handle_css(path)?,
            "sml" => Sml::handle_sml(path)?,
            _ => {}
        };
        Ok(())
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

    fn handle_css(file: PathBuf) -> Result<()> {
        let format = output::Format {
            style: output::Style::Compressed,
            .. Default::default()
        };
        let css = match compile_scss_path(&file, format) {
            Ok(v) => v,
            Err(e) => {
                println!("failed to compile scss/csss: {:#?}", e);
                return Ok(());
            },
        }; 
        println!("{:?}: {}", file, String::from_utf8(css).unwrap());
        Ok(())
    }

    fn handle_sml(file: PathBuf) -> Result<()> {
        let mut content = match read_to_string(&file) {
            Ok(v) => v,
            Err(_) => {
                println!("failed to read {:#?} to string", file);
                return Ok(());
            },
        };
        //[p] [/p]
        // normal paragraph text
        content = content.replace("[p]", "<p>").replace("[/p]", "</p>");
        //[img] [/img]
        // images
        //
        //[code] [/code]
        // TODO
        // syntax highlighting
        // code blocks
        content = content.replace("[code]", "<code>").replace("[/code]", "</code>");
        //[title] [/title]
        // page title
        content = content.replace("[title]", "<title>").replace("[/title]", "</title>");
        //[section] [/section]
        // section title
        content = content.replace("[section]", "<section>").replace("[/section]", "</section>");
        //[bold] [/bold]
        // bold text
        content = content.replace("[bold]", "<bold>").replace("[/bold]", "</bold>");
        //[comment] [/comment]
        // side comment for text
        content = content.replace("[comment]", "<comment>").replace("[/comment]", "</comment>");
        //[quote] [/quote]
        content = content.replace("[quote]", "<quote>").replace("[/quote]", "</quote>");
        //[date] [/date]
        // TODO
        // special date formatting, possibly include time in local time zone
        content = content.replace("[date]", "<date>").replace("[/date]", "</date>");
        //[footer] [/footer]
        // TODO
        // closing information and back to top button
        content = content.replace("[footer]", "footer");
        //[header] [/header]
        // bar with navbar and stuff
        content = content.replace("[header]", "<header>").replace("[/header]", "</header>");
        //[profile] [/profile]
        content = content.replace("[profile]", "profile");
        Sml::save_file(file, content)?;
        Ok(())
    }

    fn save_file(file: PathBuf, content: String) -> Result<()> {
        Ok(())
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
