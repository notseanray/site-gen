use std::io::Write;
use std::{collections::HashMap, fs::create_dir_all};
use std::fs::{read_to_string, OpenOptions};
use std::path::PathBuf;
use std::str::FromStr;
use ignore::WalkBuilder;
use notify::DebouncedEvent;
use ignore::DirEntry;
use rsass::{compile_scss_path, output};

#[derive(Clone)]
pub struct Sml {
    hashset: HashMap<PathBuf, u64>,
    new_hashset: HashMap<PathBuf, u64>,
    static_content: HashMap<PathBuf, String>,
    global_dir: PathBuf,
    data_in: PathBuf,
    data_out: PathBuf
}

impl Sml {
    pub fn new(dir_in: Option<&str>, dir_global: Option<&str>, dir_out: Option<&str>, dir_static: Option<&str>) -> Self {
        let global_dir = Sml::inspect_dir(dir_global, "./");
        let data_in = Sml::inspect_dir(dir_in, "./content");
        let data_out = Sml::inspect_dir(dir_out, "./build");
        let static_dir = Sml::inspect_dir(dir_static, "./static");
        let mut static_content = HashMap::new();
        let walker = WalkBuilder::new(static_dir).build();
        walker.for_each(|result| {
            let result = result.unwrap().into_path();
            static_content.insert(result.to_owned(), read_to_string(result).unwrap());
        });
        let folders = [&global_dir, &data_in, &data_out];
        // attempt to create the folders if they don't exist
        folders.iter().for_each(|x| {
            let _ = create_dir_all(&x);
        });
        Self {
            hashset: HashMap::new(),
            new_hashset: HashMap::new(),
            static_content,
            global_dir,
            data_in,
            data_out,
        }
    }

    pub fn update(&mut self) {
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
        let walker = WalkBuilder::new(&self.data_in).build();
        walker.for_each(|result| self.handle_file_type(result.unwrap()));
        //self.update_hashset(); 
    }

    fn handle_file_type(&self, entry: DirEntry) {
        let path = entry.into_path();
        let extension = match path.extension() {
            Some(v) => v,
            None => return
        };
        match extension.to_string_lossy().to_string().as_str() {
            "css" | "scss" => self.handle_css(path),
            "sml" => self.handle_sml(path),
            _ => {}
        };
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

    fn handle_css(&self, file: PathBuf) {
        let format = output::Format {
            style: output::Style::Compressed,
            .. Default::default()
        };
        let css = match compile_scss_path(&file, format) {
            Ok(v) => v,
            Err(e) => {
                println!("failed to compile scss/csss: {:#?}", e);
                return;
            },
        }; 
        let mut temp = Vec::new();
        temp.clone_from(&css);
        println!("{:?}: {}", file, String::from_utf8(temp).unwrap());
        self.save_file(file, &css);
    }

    fn handle_sml(&self, file: PathBuf) {
        let mut content = match read_to_string(&file) {
            Ok(v) => v,
            Err(_) => {
                println!("failed to read {:#?} to string", file);
                return;
            },
        };
        //[p] [/p]
        // normal paragraph text
        content = content.replace("[p]", "<p>").replace("[/p]", "</p>");
        //[img] [/img]
        // images
        content = content.replace("[img]", "<img src=\"").replace("[/img]", "\"></img>");
        //[codeBlock] [/codeBlock]
        // code block with no syntax highlighting
        content = content.replace("[code]", "<codeBlock>").replace("[/code]", "</codeBlock>");
        //[codeHighlight] [/codeHighlight]
        // example usage: 
        // [codeHighlight][lang]html[/lang]
        // [p]hi[/p]
        // [/codeHighlight]
        // syntax highlighting code block
        // code blocks
        content = content.replace("[codeHighlight]", "<pre><code ").replace("[/codeHighlight]", "</code></pre>");
        content = content.replace("[lang]", "class=\"language-").replace("[/lang]", "\">");
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
        self.save_file(file, content.as_bytes());
    }

    fn save_file(&self, file: PathBuf, content: &[u8]) {
        let output = format!(
            "{:#?}/{}", 
             &self.data_out.display(), 
             &file.as_path().to_str().unwrap()[self.data_in.to_str().unwrap().len()..]
        ); 
        let _ = create_dir_all(&output);
        let mut options = OpenOptions::new()
            .write(true)
            .open(&output)
            .unwrap();
        options.write_all(&content).expect("write error!");
    }

    fn update_hashset(&mut self, file: PathBuf) {
    }
}

type Ev = DebouncedEvent;

pub fn event_handler(ev: &Ev) -> bool {
    match ev {
        Ev::Write(_) => true,
        Ev::Create(_) => true,
        Ev::Remove(_) => true,
        _ => false
    }
}
