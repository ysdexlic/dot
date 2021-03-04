use std::fs::{create_dir, read_link, remove_dir, remove_file, write, File};
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use crate::utils;

pub fn get_config_dir() -> PathBuf {
    let output_dir = utils::get_output_dir();
    let mut config_dir_path = PathBuf::new();
    config_dir_path.push(format!("{}/{}", output_dir, ".dot"));
    config_dir_path
}

pub fn set_state() -> std::io::Result<()> {
    let mut links = utils::get_links();
    if links.is_err() {
        links = Ok(vec![]);
    }
    let links = links.unwrap();

    let config_dir = get_config_dir();
    let state_file_path = format!("{}/{}", config_dir.to_str().unwrap(), ".state");

    let mut filestr = String::new();
    for link in links {
        let link_str = format!("{} {}\n", link.0, link.1);
        filestr.push_str(&link_str);
    }
    write(&state_file_path, filestr)?;

    Ok(())
}

pub fn setup() -> std::io::Result<()> {
    let config_dir = get_config_dir();
    let state_file_path = format!("{}/{}", config_dir.to_str().unwrap(), ".state");

    if !Path::new(&config_dir).exists() {
        create_dir(config_dir)?;
    }

    if !Path::new(&state_file_path).exists() {
        File::create(&state_file_path)?;
    }

    Ok(())
}

pub fn get_state() -> Vec<(String, String)> {
    let config_dir = get_config_dir();
    let state_file_path = format!("{}/{}", config_dir.to_str().unwrap(), ".state");

    let file = File::open(state_file_path).expect("couldn't read file");
    let reader = BufReader::new(file);

    let mut output: Vec<(String, String)> = vec![];
    for line in reader.lines() {
        let line2 = line.unwrap();
        let paths: Vec<&str> = line2.split(' ').collect();
        let path_in = paths[0];
        let path_out = paths[1];
        output.push((path_in.to_string(), path_out.to_string()));
    }

    output
}

pub fn clean() -> std::io::Result<()> {
    let output_dir = utils::get_output_dir();
    let state = get_state();

    for links in state {
        let path_in = Path::new(&links.0);
        let path_out = Path::new(&links.1);
        let slink = read_link(&path_out);
        if !path_in.exists() && slink.is_ok() && !slink.unwrap().exists() {
            // if original path doesn't exist but symlink still exists (and is broken), remove it
            remove_file(path_out)?;

            // remove unused directories
            let out_str = path_out.to_str().unwrap();
            let path = out_str.replace(&format!("{}/", &output_dir), "");
            let sub_dirs: &[&str] = &path.split("/").collect::<Vec<&str>>();
            let sub_dirs = &sub_dirs[..sub_dirs.len() - 1];

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
    }

    Ok(())
}
