use ignore::DirEntry;
use ignore::WalkBuilder;
use notify::DebouncedEvent;
use rsass::{compile_scss_path, output};
use std::fs::copy;
use std::fs::File;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::HashMap, fs::create_dir_all};

#[derive(Clone)]
struct Template {
    name: String,
    begin: String,
    end: Option<String>,
}

#[derive(Clone)]
pub struct Sml {
    hashset: HashMap<PathBuf, u64>,
    static_content: Option<Vec<Template>>,
    data_in: PathBuf,
    data_out: PathBuf,
}

impl Sml {
    pub fn new(dir_in: Option<&str>, dir_out: Option<&str>, dir_static: Option<&str>) -> Self {
        let data_in = Sml::inspect_dir(dir_in, "./content");
        let data_out = Sml::inspect_dir(dir_out, "./build");
        let static_dir = Sml::inspect_dir(dir_static, "./static");
        // there used to be more folders to create so an array made more sense for compactness, but
        // I'll have to clean this up depending on the future changes I make
        let folders = [&data_in, &data_out];
        // attempt to create the folders if they don't exist
        folders.iter().for_each(|x| {
            let _ = create_dir_all(&x);
        });
        Self {
            hashset: HashMap::new(),
            static_content: Sml::load_templates(static_dir),
            data_in,
            data_out,
        }
    }

    pub fn update(&mut self) {
        let walker = WalkBuilder::new(&self.data_in).build();
        walker.for_each(|result| self.handle_file_type(result.unwrap()));
        self.update_hashset();
    }

    fn load_templates(static_location: PathBuf) -> Option<Vec<Template>> {
        let mut templates = Vec::new();
        let walker = WalkBuilder::new(static_location).build();
        walker.for_each(|result| {
            let result = &result.unwrap().into_path();
            let (start, end) = match read_to_string(result) {
                Ok(v) => {
                    let content: Vec<String> = v.split("[split]").map(|x| x.to_string()).collect();
                    let end = match content.len() {
                        3.. => {
                            println!("more than 3 splits detected in template: {v}");
                            None
                        }
                        2.. => Some(content[1].to_owned()),
                        _ => None,
                    };
                    (Some(content[0].to_owned()), end)
                }
                Err(_) => return,
            };
            let name = result.file_name();
            match name {
                Some(_) => {}
                None => return,
            };
            let name = result.file_name().unwrap().to_string_lossy().to_string();
            println!("found template: {name}");
            templates.push(Template {
                name,
                begin: start.unwrap(),
                end,
            });
        });
        match templates.len() {
            1.. => Some(templates),
            _ => None,
        }
    }

    fn handle_file_type(&self, entry: DirEntry) {
        let path = entry.into_path();
        let hash = &self.hashset;
        if hash.contains_key(&path)
            && hash.get_key_value(&path)
                == Some((
                    &path,
                    &seahash::hash(read_to_string(&path).unwrap().as_bytes()),
                ))
            && !path.exists()
        {
            return;
        }
        let extension = match path.extension() {
            Some(v) => v,
            None => return,
        };
        match extension.to_string_lossy().to_string().as_str() {
            "css" | "scss" => self.handle_css(path),
            "sml" => self.handle_sml(path),
            _ => self.raw_cp(path),
        };
    }

    fn raw_cp(&self, path: PathBuf) {
        let final_file = format!(
            "{}{}",
            &self.data_out.to_string_lossy().to_string(),
            &path.to_string_lossy().to_string()[self.data_in.to_string_lossy().len()..]
        );
        let folder_creation_path =
            &final_file[..final_file.len() - path.file_name().unwrap().len()];
        let _ = create_dir_all(folder_creation_path);
        copy(path, final_file).expect("failed to direct copy files");
    }

    fn inspect_dir(path: Option<&str>, default: &str) -> PathBuf {
        let entry = PathBuf::from_str(match path {
            Some(v) => v,
            None => default,
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
            ..Default::default()
        };
        let css = match compile_scss_path(&file, format) {
            Ok(v) => v,
            Err(e) => {
                println!("failed to compile scss/csss: {:#?}", e);
                return;
            }
        };
        let mut temp = Vec::new();
        temp.clone_from(&css);
        //println!("{:?}: {}", file, String::from_utf8(temp).unwrap());
        self.save_file(file, &css, "css");
    }

    fn handle_sml(&self, file: PathBuf) {
        let mut content = match read_to_string(&file) {
            Ok(v) => v,
            Err(_) => {
                println!("failed to read {:#?} to string", file);
                return;
            }
        };
        //[p] [/p]
        // normal paragraph text
        content = content
            .replace("[p]", "<p>")
            .replace("[/p]", "</p>");
        //[iframe] [/iframe]
        // embed
        content = content
            .replace("[iframe]", r#"<iframe src=""#)
            .replace("[/iframe]", r#"" frameBorder="0"></iframe>"#);
        content = content
            .replace("[utterances]", r#"<script src="https://utteranc.es/client.js" 
        repo=""#)
            .replace("[/utterances]", r#""
        issue-term="pathname"
        theme="gruvbox-dark"
        crossorigin="anonymous"
        async>
</script>"#);
        //[link] [/link]
        // hyper link
        content = content
            .replace("[link]", r#"<a href=""#)
            .replace("[,]", r#"" rel="noreferrer" target="_blank">"#)
            .replace("[/link]", "</a>");
        //[img] [/img]
        // images
        content = content
            .replace("[img]", r#"<img src=""#)
            .replace("[/img]", r#"" alt=""></img>"#);
        //[codeBlock] [/codeBlock]
        // code block with no syntax highlighting
        content = content
            .replace("[code]", "<codeBlock>")
            .replace("[/code]", "</codeBlock>");
        //[codeHighlight] [/codeHighlight]
        // example usage:
        // [codeHighlight][lang]html[/lang]
        // [p]hi[/p]
        // [/codeHighlight]
        // syntax highlighting code block
        // code blocks
        content = content
            .replace("[codeHighlight]", "<pre><code ")
            .replace("[/codeHighlight]", "</code></pre>");
        content = content
            .replace("[lang]", r#"class="language-"#)
            .replace("[/lang]", r#"">"#);
        //[profile] [/profile]
        content = content.replace("[profile]", "profile");
        //[title] [/title]
        // page title
        content = content
            .replace("[title]", "<title>")
            .replace("[/title]", "</title>");
        //[section] [/section]
        // section title
        content = content
            .replace("[section]", "<section>")
            .replace("[/section]", "</section>");
        //[bold] [/bold]
        // bold text
        content = content
            .replace("[bold]", "<bold>")
            .replace("[/bold]", "</bold>");
        //[comment] [/comment]
        // side comment for text
        content = content
            .replace("[comment]", "<comment>")
            .replace("[/comment]", "</comment>");
        //[quote] [/quote]
        content = content
            .replace("[quote]", "<quote>")
            .replace("[/quote]", "</quote>");
        //[date] [/date]
        // TODO
        // special date formatting, possibly include time in local time zone
        content = content
            .replace("[date]", "<date>")
            .replace("[/date]", "</date>");
        //[footer] [/footer]
        // TODO
        // closing information and back to top button
        content = content
            .replace("[footer]", "<footer>")
            .replace("[/footer]", "</footer>");
        //[header] [/header]
        // bar with navbar and stuff
        content = content
            .replace("[header]", "<header>")
            .replace("[/header]", "</header>");
        // explicit new line support
        content = content.replace("[n]", "<br>");
        content = content.replace(
            "[btt-button]",
            r#"<button onclick="topFunction()" title="">↑ back to top</button>
<script>
function topFunction() {
	document.body.scrollTop = 0;
	document.documentElement.scrollTop = 0;
}
</script>
"#,
        );
        self.save_file(file, content.as_bytes(), "html");
    }

    // EXTREMELY janky, need to fix asap
    fn save_file(&self, file: PathBuf, content: &[u8], file_type: &str) {
        let mut final_content = String::from_utf8(content.to_vec()).unwrap();
        let file_name = match file.file_name() {
            Some(v) => v.to_string_lossy().to_string(),
            None => return,
        };
        let content = &self.static_content;
        if content.is_none() {
            return;
        }
        for template in content.as_deref().unwrap() {
            if template.name == "all"
                && file.extension().is_some()
                && file.extension().unwrap() == "sml"
            {
                final_content = format!("{}{}", template.begin.clone(), final_content);
                if template.end.is_none() {
                    continue;
                }
                let end = template.end.as_ref().unwrap();
                final_content.push_str(&end);
            }
            if file_name.len() < template.name.len()
                || &file_name[..template.name.len()] != template.name
            {
                continue;
            }
            final_content = format!("{}{}", template.begin.clone(), final_content);
            if template.end.is_none() {
                continue;
            }
            let end = template.end.as_ref().unwrap();
            final_content.push_str(&end);
        }
        if let Some(v) = file.extension() {
        let final_file_name = file.as_path().to_str().unwrap();
            let output = format!(
                "{}/{}{}",
                &self.data_out.display().to_string(),
                &final_file_name
                    [self.data_in.to_str().unwrap().len()..final_file_name.len() - v.len()],
                file_type
            );
            println!("built {output}");
            let folder_creation_path = &output[..output.len() - (file_name.len() + 1)];
            let _ = create_dir_all(folder_creation_path);
            let _ = File::create(&output);
            let mut options = OpenOptions::new().write(true).open(&output).unwrap();
            options
                .write_all(&final_content.as_bytes())
                .expect("write error!");
            
        }
    }

    fn update_hashset(&mut self) {
        let walker = WalkBuilder::new(&self.data_in).build();
        let mut new_hashes = HashMap::new();
        walker.for_each(|result| {
            if let Ok(v) = result {
                let path = v.into_path();
                if path.is_dir() {
                    return;
                }
                if let Ok(x) = read_to_string(&path) {
                    new_hashes.insert(
                        path.to_owned(),
                        seahash::hash(x.as_bytes()),
                    );
                }
            }
        });
        self.hashset = new_hashes;
    }
}

type Ev = DebouncedEvent;

pub fn event_handler(ev: &Ev) -> bool {
    match ev {
        Ev::Write(_) => true,
        Ev::Create(_) => true,
        Ev::Remove(_) => true,
        _ => false,
    }
}
