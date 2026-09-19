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

fn unique_destination(destination: PathBuf) -> PathBuf {
    if !destination.exists() {
        return destination;
    }

    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    let stem = destination
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("file");
    let extension = destination.extension().and_then(|value| value.to_str());

    for counter in 1.. {
        let file_name = match extension {
            Some(ext) if !ext.is_empty() => format!("{stem}_{counter}.{ext}"),
            _ => format!("{stem}_{counter}"),
        };
        let candidate = parent.join(file_name);

        if !candidate.exists() {
            return candidate;
        }
    }

    unreachable!()
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

        let destination = unique_destination(destination_folder.join(file_name));

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
