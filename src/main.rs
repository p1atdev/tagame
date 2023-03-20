use clap::{Parser, Subcommand};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static IMAGE_EXT: [&str; 5] = ["jpg", "jpeg", "png", "gif", "webp"];

// Parserを継承した構造体はArgの代わりに使用することが可能。
#[derive(Parser)]
#[clap(
    name = "Tagame",
    author = "Plat",
    version = "v0.1.0",
    about = "Generate or manage tag files"
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
    // Replcae {
    //     // input dir
    //     input: String,

    //     // tag to replace from
    //     from: String,

    //     // tag to replace to
    //     to: String,

    //     #[clap(short = 'e', long = "extension", default_value = "txt")]
    //     extension: String,
    // },
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
        // Action::Replcae {
        //     input,
        //     from,
        //     to,
        //     extension,
        // } => println!("Hello, {}!", input),
    }
}

// create text file
fn create_text_file(path_str: &String, content: String) {
    let path = Path::new(path_str);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    // ファイルを書き込み専用モードで開く。返り値は`io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
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

fn generate_tag_files(input: &String, tag: &String, output: &Option<String>, extension: &String) {
    let images = get_images_in(input);
    for image in images {
        let image_file_name = Path::new(&image).file_name().unwrap().to_str().unwrap();
        let mut tag_file_path = String::new();
        if output.is_some() {
            tag_file_path.push_str(output.as_ref().unwrap());
            tag_file_path.push_str("/");
        } else {
            tag_file_path.push_str(input);
            tag_file_path.push_str("/");
        }
        tag_file_path.push_str(image_file_name);
        tag_file_path.push_str(".");
        tag_file_path.push_str(extension);

        let content = tag.clone();
        create_text_file(&tag_file_path, content);

        println!("{}", image_file_name);
    }
}
