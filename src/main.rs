use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn category(extension: &str) -> &str {
    match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" => "Images",
        "pdf" | "doc" | "docx" | "txt" | "ppt" | "pptx" => "Documents",
        "mp4" | "mkv" | "avi" | "mov" => "Videos",
        "mp3" | "wav" | "aac" | "flac" => "Audio",
        "zip" | "rar" | "7z" => "Archives",
        _ => "Other",
    }
}

fn organize_folder(folder: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let folder_name = category(extension);
        let destination_folder: PathBuf = folder.join(folder_name);

        fs::create_dir_all(&destination_folder)?;

        let file_name = match path.file_name() {
            Some(name) => name,
            None => continue,
        };

        let destination = destination_folder.join(file_name);

        fs::rename(&path, &destination)?;

        println!(
            "Moved {} -> {}",
            path.display(),
            destination.display()
        );
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rustysort <folder>");
        return;
    }

    let folder = Path::new(&args[1]);

    if !folder.exists() {
        println!("Folder does not exist.");
        return;
    }

    match organize_folder(folder) {
        Ok(_) => println!("Folder organized successfully."),
        Err(error) => println!("Error: {}", error),
    }
}
