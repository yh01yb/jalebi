use std::env;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use zip::ZipArchive;

fn main() {
    println!("BUILDING...\n");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rust_unzip <file.zip>");
        return;
    }

    let zip_path = Path::new(&args[1]);

    if !zip_path.exists() {
        eprintln!("Error: ZIP file does not exist.");
        return;
    }

    match unzip_file(zip_path) {
        Ok(_) => println!("\nCOMPLETED"),
        Err(e) => eprintln!("Failed: {}", e),
    }
}

fn unzip_file(zip_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    let extract_dir = zip_path.parent().unwrap_or(Path::new("."));

    for i in 0..archive.len() {
        let mut zipped_file = archive.by_index(i)?;

        
        let file_name = zipped_file.name().to_string();
        let outpath = sanitize_path(extract_dir, &file_name)?;

        if zipped_file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut outfile = File::create(&outpath)?;
            io::copy(&mut zipped_file, &mut outfile)?;
        }

        println!("Extracting: {}", file_name);
    }

    Ok(())
}

fn sanitize_path(base: &Path, name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let full_path = base.join(name);

    if !full_path.starts_with(base) {
        return Err("Invalid ZIP path detected".into());
    }

    Ok(full_path)
}
