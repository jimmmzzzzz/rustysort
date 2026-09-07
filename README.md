# RustySort

RustySort is a lightweight command-line file organizer built with Rust. It scans a folder and automatically sorts files into categories based on their file extensions.

## Features

- Organizes files automatically
- Supports images, documents, videos, audio files, and archives
- Places unsupported file types inside an `Other` folder
- Creates category folders automatically
- Uses only Rust's standard library
- Works from the command line

## Example

Before:

```text
Downloads/
├── photo.png
├── notes.pdf
├── song.mp3
├── movie.mp4
├── project.zip
└── random.xyz
```

After running RustySort:

```text
Downloads/
├── Images/
│   └── photo.png
├── Documents/
│   └── notes.pdf
├── Audio/
│   └── song.mp3
├── Videos/
│   └── movie.mp4
├── Archives/
│   └── project.zip
└── Other/
    └── random.xyz
```

## Requirements

Install Rust and Cargo from:

https://www.rust-lang.org/tools/install

Check your installation:

```bash
rustc --version
cargo --version
```

## Run the Project

Clone the repository:

```bash
git clone https://github.com/YOUR-USERNAME/rustysort.git
cd rustysort
```

Run RustySort and provide the folder you want to organize:

### Windows

```bash
cargo run -- "C:\Users\YourName\Downloads"
```

### macOS / Linux

```bash
cargo run -- "/home/yourname/Downloads"
```

## Build

Create a release build:

```bash
cargo build --release
```

The compiled program will be located inside:

```text
target/release/
```

## File Categories

| Category | Extensions |
| --- | --- |
| Images | jpg, jpeg, png, gif, webp |
| Documents | pdf, doc, docx, txt, ppt, pptx |
| Videos | mp4, mkv, avi, mov |
| Audio | mp3, wav, aac, flac |
| Archives | zip, rar, 7z |
| Other | Any unsupported extension |

## Project Structure

```text
rustysort/
├── src/
│   └── main.rs
├── .gitignore
├── Cargo.toml
└── README.md
```

## Built With

- Rust
- Rust Standard Library

## License

This project is open source and available for learning and personal use.
