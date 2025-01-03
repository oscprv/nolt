use std::fs::{File, create_dir_all};
use std::io::{self, Write};
use std::path::{Path};

fn main() {
    let folder_path = get_saved_folder_path(); 

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

fn get_saved_folder_path() -> String {
    let config_file = "nolt_config.txt";
    if Path::new(config_file).exists() {
        let saved_path = std::fs::read_to_string(config_file).expect("Failed to read config file");
        saved_path.trim().to_string()
    } else {
        println!("No config file found. Add a folder path where files will be created:");
        let mut folder = String::new();
        io::stdin().read_line(&mut folder).expect("Failed to read line");
        let folder = folder.trim();

        save_folder_path(folder);

        folder.to_string()
    }
}

fn save_folder_path(folder: &str) {
    let mut file = File::create("nolt_config.txt")
        .expect("Failed to create config file");
    writeln!(file, "{}", folder).expect("Failed to write to config file");
}
