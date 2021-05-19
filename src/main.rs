#![allow(unused, dead_code)]

use std::env;
use walkdir::WalkDir;

fn main() {
   // user gets the directory name from the cli. It could also
  8     // list of directories not just a single one. 
  9     // if non is provided the `current` directory is used as default
 10     let mut filename = env::args().skip(1).collect::<Vec<_>>();
 11     match filename.len() {
 12         0 => parse_directory_tree("."),
 13         _ => filename
 14             .into_iter()
 15             .for_each(|filename| parse_directory_tree(&filename)),
 16     }
}

/// `parse_directory_tree` function takes a string and return ().
/// using `WalkDir` from `walkdir` it walks through the directory
/// provided and list of contents of directories in a tree-like format.
/// It is works like `tree` command on *nix like system.  
fn parse_directory_tree(dirname: &str) {
    let mut number_of_dirs = 0;
    let mut number_of_files = 0;

    WalkDir::new(dirname).into_iter().for_each(|entry| {
        let file = entry.expect("no entry..");
        let depth = &file.path().components().count();
        if file.file_type().is_dir() {
            let name = file.file_name().to_string_lossy();
            if name == "." || name == ".." {
                number_of_dirs -= 1;
            } else {
                number_of_dirs += 1;
            }
            println!(
                "{}  |  \\__ {}/",
                "  |".repeat(*depth),
                file.file_name().to_string_lossy()
            );
        } else {
            number_of_files += 1;
            println!(
                "{}  |-- {}",
                "  |".repeat(*depth + 1),
                &file.file_name().to_string_lossy()
            );
        }
    });

    println!(
        "Summary: {} {directories}, {} {files}",
        number_of_dirs + 1,
        number_of_files,
        directories = match (number_of_dirs + 1) {
            1 => "directory",
            _ => "directories",
        },
        files = match number_of_files {
            1 => "file",
            _ => "files",
        }
    );
}
