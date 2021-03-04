use glob::glob;
use regex::Regex;
use serde_derive::Deserialize;
use std::env;
use std::fs::{metadata, File};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn get_output_dir() -> String {
    let out_dir = format!(
        "{}/dev/dot/example_out",
        dirs::home_dir().unwrap().to_str().unwrap()
    );
    // let out_dir = format!("{}", dirs::home_dir().unwrap().to_str().unwrap());
    out_dir
}

#[derive(Deserialize)]
pub struct Config {
    excludes: Vec<String>,
    dotfile_dirs: Vec<String>,
}

pub fn get_config() -> std::io::Result<Config> {
    let out_dir = get_output_dir();

    let dotrc_env_var = env::var("DOTRC");
    let dotrc_env_var_set = dotrc_env_var.is_ok();

    let mut dotrc_path: PathBuf = PathBuf::new();

    if !dotrc_env_var_set {
        // DOTRC env var not set, use dot.toml from $HOME
        dotrc_path.push(&format!("{}/.dotrc", out_dir));
    } else {
        // DOTRC env var set, use env var path
        dotrc_path.push(dotrc_env_var.unwrap());
    }

    let mut file = File::open(dotrc_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let config: Config = toml::from_str(&contents).unwrap();
    Ok(config)
}

// Returns array of (to, from) strings to link
pub fn get_links() -> std::io::Result<Vec<(String, String)>> {
    let config = get_config()?;

    let mut links: Vec<(String, String)> = vec![];

    for mut dir_str in config.dotfile_dirs {
        dir_str = dir_str.replace("$HOME", dirs::home_dir().unwrap().to_str().unwrap());
        dir_str = dir_str.replace("~", dirs::home_dir().unwrap().to_str().unwrap());

        let globstr = format!("{}/**/*", &dir_str);

        for path in glob(&globstr).expect("something went wrong globbing") {
            let path = path.unwrap();
            let path_str = path.display().to_string();

            let path_names: Vec<&str> = path_str.split(&format!("{}/", &dir_str)).collect();
            let path_name = path_names[1];

            if metadata(&path_str).unwrap().is_dir() {
                continue;
            }

            let sub_dirs: &[&str] = &path_name.split("/").collect::<Vec<&str>>();
            let mut has_dot = false;
            for i in 0..sub_dirs.len() {
                let p = format!("{}", sub_dirs[i]);
                if p.chars().nth(0).unwrap() == '.' {
                    has_dot = true;
                }
            }

            if has_dot {
                continue;
            }

            let mut should_exclude: bool = false;
            for exclude in &config.excludes {
                let regex = format!(r"{}", &exclude);
                let re = Regex::new(&regex).unwrap();
                if re.is_match(&path_str) {
                    should_exclude = true;
                    break;
                }
            }
            if should_exclude {
                continue;
            }

            let out_path = &format!("{}/.{}", get_output_dir(), path_name);
            links.push((path_str, out_path.to_string()));
        }
    }
    Ok(links)
}
