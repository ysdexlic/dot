use colour::e_red_ln;
use std::fs::{create_dir, read_link, remove_file, rename, symlink_metadata};
use std::os::unix::fs::symlink;
use std::path::Path;

use crate::utils;

pub fn link_dotfiles(should_bootstrap: bool) -> std::io::Result<()> {
    if should_bootstrap {
        utils::bootstrap()?;
    }

    let links = utils::get_links();

    if links.is_err() {
        // config file can't be found
        e_red_ln!("No .dotrc found in home directory, for initial call use the DOTRC env variable");
        print!("Example: ");
        println!("DOTRC=$HOME/path/to/.dotrc dot");
        return Ok(());
    }

    let links = links.unwrap();

    for link in links {
        let (link_in, link_out) = link;
        if Path::new(&link_out).exists() {
            let md = symlink_metadata(&link_out)?;
            let file_type = md.file_type();
            if file_type.is_symlink() {
                if read_link(&link_out).unwrap().to_str().unwrap() == link_in {
                    continue;
                }
                remove_file(&link_out)?;
            }
            if file_type.is_file() {
                let new_out_path = &format!("{}.old", link_out);
                rename(&link_out, new_out_path)?;
            }
        } else {
            mkdirs(&link_out)?;
        }
        symlink(&link_in, &link_out)?;
    }

    Ok(())
}

fn mkdirs(link_out: &str) -> std::io::Result<()> {
    let output_dir = utils::get_output_dir();
    let path = link_out.replace(&format!("{}/", &output_dir), "");

    let sub_dirs: &[&str] = &path.split("/").collect::<Vec<&str>>();

    let mut s = String::from(&output_dir);

    // always going to be at least length 1
    // this loops through only directories
    for i in 0..sub_dirs.len() - 1 {
        let p = format!("{}", sub_dirs[i]);
        s.push_str(&format!("/{}", &p));
        let p_out = Path::new(&s);
        if !p_out.exists() {
            create_dir(p_out)?;
        }
    }

    Ok(())
}
