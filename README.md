# Dot

[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/ysdexlic/dot/build/master)](https://github.com/ysdexlic/dot/actions)
[![crate](https://img.shields.io/crates/v/dotm.svg?color=brightgreen)](https://crates.io/crates/dotm)
![license](https://img.shields.io/crates/l/dotm)


## Installation

Install via Cargo:
```bash
cargo install dotm
```

Install via Homebrew:
```bash
brew install ysdexlic/formulae/dot
```

## TODO

- [ ] **Write tests**
- [ ] Error handling
  - [x] Add error when config file isn't set
  - [ ] Fix repeated use of printing the "No .dotrc found" error
  - [ ] Find out best error handling practices
  - [ ] Return pretty errors
- [x] Add github actions to build binary
  - [x] Add binary to releases
- [x] Release on homebrew
  - [x] Automate homebrew update PRs (`mislav/bump-homebrew-formula-action`)


- [x] Don't copy pesky system files like `.DS_Store` (done by only symlinking
  non dotted files)
  - [x] Seems to still be happening for nested dotfiles - i.e.
        `something/something/.DS_Store`
- [x] Copy folders/files recursively
  - [x] Don't make a symlink for the file, instead make the directories and only
    symlink the file


- [x] Add bootstrapping feature (run bootstrap executables)
- [ ] Add ability to have pre/post hooks
- [ ] Add ability to have host or tag specific files


- [x] Add regex file exclusions

- [ ] Add init command to create a new dotfile repo
- [ ] Add clone command to wrap git & pull
  - [ ] When cloning, prompt to bootstrap
  - [ ] When cloning, prompt to add any tags
    - [ ] Go through all files and prompt to install any tags that are present
- [x] Add down command to remove symlinks created by dot
  - [x] Down command should remove empty folders created by dot
- [x] Add list command to list symlinks created by dot


- [x] Save last state after upping
- [x] Consolidate with previous state to clean when upping
  - [x] Clean should remove empty folders created by dot
