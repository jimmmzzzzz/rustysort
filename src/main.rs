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

fn is_hidden_file(path: &Path) -> bool {
    path.file_name()
        .map(|name| name.to_string_lossy().starts_with('.'))
        .unwrap_or(false)
}

fn organize_folder(folder: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        // Hidden dotfiles often hold configuration or secrets; leave them in place.
        if is_hidden_file(&path) || !path.is_file() {
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

fn run(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: rustysort <folder>".to_string());
    }

    let folder = Path::new(&args[1]);

    if !folder.exists() {
        return Err("Folder does not exist.".to_string());
    }

    organize_folder(folder).map_err(|error| format!("Error: {}", error))?;
    println!("Folder organized successfully.");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(error) = run(&args) {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{is_hidden_file, run};
    use std::path::Path;

    #[test]
    fn preserves_hidden_configuration_files() {
        assert!(is_hidden_file(Path::new(".env")));
        assert!(is_hidden_file(Path::new("Downloads/.gitignore")));
        assert!(is_hidden_file(Path::new("Downloads/.config.json")));
        assert!(!is_hidden_file(Path::new("Downloads/report.pdf")));
        assert!(!is_hidden_file(Path::new("Downloads/photo.png")));
    }

    #[test]
    fn missing_folder_argument_returns_an_error() {
        assert!(run(&["rustysort".to_string()]).is_err());
    }

    #[test]
    fn empty_folder_argument_returns_an_error() {
        assert!(run(&["rustysort".to_string(), String::new()]).is_err());
    }
}
