extern crate fs_extra;

use fs_extra::{dir::move_dir , file::move_file};
use std::path::Path;
use std::io::{stdin, stdout};
use std::io::Write;


fn mover(path_to_item: &str, new_path: &str) -> std::io::Result<()> {
    if Path::new(new_path).exists() == false {
        std::fs::create_dir(new_path)?;
    }

    // Finding some informations about file/directory.
    let metadata = std::fs::metadata(path_to_item)?;
    let file_type = metadata.file_type();
    let item_name = Path::new(path_to_item)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    let new_path: String = String::from(new_path) + &item_name;

    if file_type.is_dir() == true {
        if !Path::new(&new_path).exists() {
            let options = fs_extra::dir::CopyOptions::new();
            move_dir(path_to_item, new_path, &options);
        } else {

        }
    } else {
        if !Path::new(&new_path).exists() {
            let options = fs_extra::file::CopyOptions::new();
            move_file(path_to_item, new_path, &options);
        } else {
            if Path::new(&new_path).exists() == true {
                let mut what_to_do: String = String::new();
                print!("1: skip moving this file.\n");
                print!("2: move and replace.\n");
                print!("3: move with new name.\n");
                print!("{} is exists. What I must to do? Please enter with number: ", item_name);
                stdout().flush().unwrap();
                stdin()
                    .read_line(&mut what_to_do);

                let what_to_do = what_to_do.parse::<u8>();

                if what_to_do == 1 {

                }
            }
        }
    }

    Ok(())
}


fn main() {
    mover("/Users/mobas/Desktop/goodbye.txt", "/Users/mobas/Desktop/Test_directory/");
}
