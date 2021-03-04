use std::fs::{remove_dir, remove_file};
use std::path::Path;

use colour::{e_red_ln, green_ln};

use crate::utils;

pub fn down_dotfiles(_dot_dir: Option<String>) -> std::io::Result<()> {
    let output_dir = utils::get_output_dir();
    let links = utils::get_links();

    if links.is_err() {
        // config file can't be found
        e_red_ln!("No .dotrc found in home directory, for initail call use the DOTRC env variable");
        print!("Example: ");
        println!("DOTRC=$HOME/path/to/.dotrc dot");
        return Ok(());
    }

    let links = links.unwrap();

    for link in links {
        let (_, link_out) = link;
        let path = link_out.replace(&format!("{}/", &output_dir), "");
        let sub_dirs: &[&str] = &path.split("/").collect::<Vec<&str>>();
        let sub_dirs = &sub_dirs[..sub_dirs.len() - 1];

        if Path::new(&link_out).exists() {
            remove_file(&link_out)?;
        }

        for i in 0..sub_dirs.len() {
            let index = sub_dirs.len() - i;
            let path = &sub_dirs[..index].join("/");
            let path = format!("{}/{}", &output_dir, path);
            let path = Path::new(&path);
            if path.exists() {
                let is_empty = path.read_dir()?.next().is_none();
                if is_empty {
                    remove_dir(path)?;
                }
            }
        }
    }

    green_ln!("Removed all dotfiles");
    println!("If you want to re-link them you'll have to use the DOTRC env variable again");
    print!("Example: ");
    println!("DOTRC=$HOME/path/to/.dotrc dot");

    Ok(())
}
