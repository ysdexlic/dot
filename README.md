# Dot

### TODO

- Learn Rust
- Error handling
- Don't copy pesky system files like `.DS_Store`
- Copy folders/files recursively
  - Don't make a symlink for the file, instead make the directories and only
    symlink the file.

- Add bootstrapping feature
- Add ability to have pre/post hooks
- Add ability to have host or tag specific files

- Add down command to remove symlinks created by dot
- Add list command to list symlinks created by dot

- Dotfile watching (adds / removes symlinks) ?
  - Save last state (incase watcher goes down)
