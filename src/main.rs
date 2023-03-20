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
    version = "v0.1.2",
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
}
