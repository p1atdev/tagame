use clap::{Parser, Subcommand};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static IMAGE_EXT: [&str; 5] = ["jpg", "jpeg", "png", "gif", "webp"];

// Parserを継承した構造体はArgの代わりに使用することが可能。
#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct AppArg {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Generate {
        // input dir
        input: String,

        // tag to set
        tag: String,

        // output (optional)
        #[clap(short = 'o', long = "output")]
        output: Option<String>,

        #[clap(short = 'e', long = "extension", default_value = "txt")]
        extension: String,
    },
    Replcae {
        // input dir
        input: String,

        // tag to replace from
        from: String,

        // tag to replace to
        to: String,

        #[clap(short = 'e', long = "extension", default_value = "txt")]
        extension: String,
    },
    Insert {
        #[clap(subcommand)]
        position: InsertPosition,
    },
}

#[derive(Subcommand)]
enum InsertPosition {
    After {
        // input dir
        input: String,

        // tag to insert
        tag: String,

        // insert after
        after: String,

        #[clap(short = 'e', long = "extension", default_value = "txt")]
        extension: String,
    },
    Before {
        // input dir
        input: String,

        // tag to insert
        tag: String,

        // insert before
        before: String,

        #[clap(short = 'e', long = "extension", default_value = "txt")]
        extension: String,
    },
    Start {
        // input dir
        input: String,

        // tag to insert
        tag: String,

        #[clap(short = 'e', long = "extension", default_value = "txt")]
        extension: String,
    },
    End {
        // input dir
        input: String,

        // tag to insert
        tag: String,

        #[clap(short = 'e', long = "extension", default_value = "txt")]
        extension: String,
    },
}

enum InsertMode {
    After(String),
    Before(String),
    Start,
    End,
}

fn main() {
    let arg: AppArg = AppArg::parse();
    match arg.action {
        Action::Generate {
            input,
            tag,
            output,
            extension,
        } => generate_tag_files(&input, &tag, &output, &extension),
        Action::Replcae {
            input,
            from,
            to,
            extension,
        } => replace_tags(input, &from, &to, extension),
        Action::Insert { position } => match position {
            InsertPosition::After {
                input,
                tag,
                after,
                extension,
            } => insert_tag(input, &tag, &InsertMode::After(after), extension),
            InsertPosition::Before {
                input,
                tag,
                before,
                extension,
            } => insert_tag(input, &tag, &InsertMode::Before(before), extension),
            InsertPosition::Start {
                input,
                tag,
                extension,
            } => insert_tag(input, &tag, &InsertMode::Start, extension),
            InsertPosition::End {
                input,
                tag,
                extension,
            } => insert_tag(input, &tag, &InsertMode::End, extension),
        },
    }
}

// create text file
fn create_text_file(path: &Path, content: String) -> Result<(), String> {
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => return Err(format!("couldn't create {}: {}", display, why)),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => return Err(format!("couldn't write to {}: {}", display, why)),
        Ok(_) => return Ok(println!("successfully wrote to {}", display)),
    }
}

// read text file
fn read_text_file(path: &Path) -> Result<String, String> {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => return Err(format!("couldn't open {}: {}", display, why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => return Err(format!("couldn't read {}: {}", display, why)),
        Ok(_) => return Ok(s),
    }
}

fn get_images_in(path_str: &String) -> Vec<String> {
    let paths = fs::read_dir(path_str).unwrap();
    let mut images: Vec<String> = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap().to_string();
        let ext = path
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .to_lowercase();
        if IMAGE_EXT.contains(&ext.as_str()) {
            images.push(path_str);
        }
    }
    return images;
}

fn get_tag_files_in(path_str: String, extension: String) -> Vec<String> {
    let paths = fs::read_dir(path_str).unwrap();
    let mut files: Vec<String> = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap().to_string();
        let ext = path
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .to_lowercase();
        if ext.as_str() == extension {
            files.push(path_str);
        }
    }
    return files;
}

fn generate_tag_files(input: &String, tag: &String, output: &Option<String>, extension: &String) {
    let images = get_images_in(input);
    for image in images {
        // without ext
        let mut image_file_name = Path::new(&image)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        image_file_name.push_str(".");
        image_file_name.push_str(extension);

        let mut tag_file_path_str = String::new();
        if output.is_some() {
            tag_file_path_str.push_str(output.as_ref().unwrap());
        } else {
            tag_file_path_str.push_str(input);
        }
        let tag_file_path = Path::new(&tag_file_path_str).join(&image_file_name);

        let content = tag.clone();
        match create_text_file(&tag_file_path, content) {
            Ok(_) => {}
            Err(error) => {
                // error message
                println!("{}. Skipped", error);
            }
        }

        println!("{}", image_file_name);
    }

    println!("Done!")
}

// read all tag files and replace tags
fn replace_tags(input: String, from: &String, to: &String, extension: String) {
    let txt_files = get_tag_files_in(input, extension);

    for txt_file in txt_files {
        let content = read_text_file(&Path::new(&txt_file)).unwrap();
        let replaced_content = content.replace(from, to);
        let txt_file_stem = Path::new(&txt_file)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        println!("{}: {}", txt_file_stem, replaced_content);
    }

    println!("Done!")
}

fn insert_tag(input: String, tag: &String, mode: &InsertMode, extension: String) {
    let txt_files = get_tag_files_in(input, extension);

    for txt_file in txt_files {
        let content = read_text_file(&Path::new(&txt_file)).unwrap();

        let mut inserted_content = String::new();

        match mode {
            InsertMode::After(after) => {
                // TODO: insert tag after
            }
            InsertMode::Before(before) => {
                // TODO: insert tag before
            }
            InsertMode::Start => {
                // insert tag at start
                inserted_content.push_str(tag);
                inserted_content.push_str(&content);
            }
            InsertMode::End => {
                // insert tag at end
                inserted_content.push_str(&content);
                inserted_content.push_str(tag);
            }
        }

        let txt_file_stem = Path::new(&txt_file)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        println!("{}: {}", txt_file_stem, inserted_content);
    }

    println!("Done!")
}
