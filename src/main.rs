use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    for _ in 0..args.count {
        println!("Counter is: {}", args.count);
    }
}

// TODO: Copy recursively the folders and files
// use std::fs;
// use std::io::{self, ErrorKind};
// use std::path::{Path, PathBuf};

// fn copy_dir_recursively(src: &Path, dest: &Path) -> io::Result<()> {
//     if !src.is_dir() {
//         return Err(io::Error::new(ErrorKind::Other, "Source path is not a directory"));
//     }

//     if !dest.exists() {
//         fs::create_dir_all(dest)?;
//     }

//     for entry in fs::read_dir(src)? {
//         let entry = entry?;
//         let path = entry.path();
//         let file_name = path.file_name().unwrap().to_owned();
//         let dest_path = dest.join(&file_name);

//         if path.is_dir() {
//             copy_dir_recursively(&path, &dest_path)?;
//         } else {
//             fs::copy(&path, &dest_path)?;
//         }
//     }

//     Ok(())
// }

// fn main() -> io::Result<()> {
//     let src = Path::new("path/to/source/folder");
//     let dest = Path::new("path/to/destination/folder");

//     copy_dir_recursively(&src, &dest)?;

//     println!("Folder copied successfully!");
//     Ok(())
// }
