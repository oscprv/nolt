use std::fs::{File, create_dir_all};
use std::io::{self, Write};
use std::path::{Path};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    folder_path: String,
}

fn main() {
    let folder_path = match get_saved_folder_path() {
        Some(path) => path,
        None => {
            println!("Please provide a folder path where files will be created:");
            let mut folder = String::new();
            io::stdin().read_line(&mut folder).expect("Failed to read line");
            let folder = folder.trim();
            if !is_valid_path(folder) {
                eprintln!("Invalid folder path provided.");
                std::process::exit(1);
            }
            save_folder_path(folder);
            folder.to_string()
        }
    };

    create_dir_all(&folder_path).expect("Failed to create folder");

    println!("Enter the filename you want to create:");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read line");
    let filename = filename.trim();

    let file_path = Path::new(&folder_path).join(filename);

    match File::create(&file_path) {
        Ok(mut file) => {
            println!("File created successfully at {}", file_path.display());
            println!("Add text to the file {}: ", filename);
            let mut text = String::new();
            io::stdin().read_line(&mut text).expect("Failed to read line");

            let _ = writeln!(file, "{}", text);
            println!("Text added to the file.");
        }
        Err(e) => println!("Error creating the file: {}", e),
    }
}

fn is_valid_path(path: &str) -> bool {
    !path.is_empty() && !path.contains("..")
}

fn get_saved_folder_path() -> Option<String> {
    let config_file = "nolt_config.yml";
    if Path::new(config_file).exists() {
        let config_file_content = std::fs::read_to_string(config_file).expect("Failed to read config file");
        let config: Config = serde_yaml::from_str(&config_file_content).expect("Failed to parse YAML");
        return Some(config.folder_path);
    }
    None
}

fn save_folder_path(folder: &str) {
    let config = Config {
        folder_path: folder.to_string(),
    };

    let config_file = "nolt_config.yml";
    let config_file_content = serde_yaml::to_string(&config).expect("Failed to serialize config");

    let mut file = File::create(config_file).expect("Failed to create config file");
    file.write_all(config_file_content.as_bytes())
        .expect("Failed to write to config file");

    println!("YAML configuration file '{}' created with the folder path.", config_file);
}
