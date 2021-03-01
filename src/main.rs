use regex::Regex;
use serde_derive::Deserialize;
use std::env;
use std::fs::{read_dir, remove_file, rename, symlink_metadata, File};
use std::io::prelude::*;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    #[derive(Deserialize)]
    struct Config {
        excludes: Vec<String>,
        dotfile_dirs: Vec<String>,
    }

    let out_dir = "/Users/davidthompson/dev/dot/example_out";

    let dotrc_env_var = env::var("DOTRC");
    let dotrc_env_var_set = dotrc_env_var.is_ok();

    let mut dotrc_path: PathBuf = PathBuf::new();

    if !dotrc_env_var_set {
        // DOTRC env var not set
        dotrc_path.push(&format!(
            "{}/.dot.toml",
            dirs::home_dir().unwrap().display()
        ));
    } else {
        dotrc_path.push(dotrc_env_var.unwrap());
    }

    let mut file = File::open(dotrc_path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let config: Config = toml::from_str(&contents).unwrap();

    for mut dir_str in config.dotfile_dirs {
        dir_str = dir_str.replace("$HOME", dirs::home_dir().unwrap().to_str().unwrap());
        dir_str = dir_str.replace("~", dirs::home_dir().unwrap().to_str().unwrap());
        let paths = read_dir(dir_str).unwrap();

        for path in paths {
            let path_str = path.unwrap().path().display().to_string();
            let mut should_exclude: bool = false;
            for exclude in &config.excludes {
                let re = Regex::new(&exclude).unwrap();
                if re.is_match(&path_str) {
                    should_exclude = true;
                    break;
                }
            }
            if should_exclude {
                continue;
            }
            let filename = Path::new(&path_str).file_name().unwrap().to_str().unwrap();
            let out_path = &format!("{}/.{}", out_dir, filename);
            if Path::new(&out_path).exists() {
                let metadata = symlink_metadata(&out_path)?;
                let file_type = metadata.file_type();
                if file_type.is_symlink() {
                    remove_file(&out_path)?;
                }
                if file_type.is_file() {
                    let new_out_path = &format!("{}.old", out_path);
                    rename(&out_path, new_out_path)?;
                }
            }
            symlink(&path_str, &out_path)?;
        }
    }

    println!("All Done!");
    Ok(())
}
