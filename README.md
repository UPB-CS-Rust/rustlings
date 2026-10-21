# Rustlings (UPB edition) 🦀

Small exercises to get you used to reading and writing [Rust](https://www.rust-lang.org) code - _Recommended in parallel to reading [the official Rust book](https://doc.rust-lang.org/book) 📚️_

## Install prerequisites

To be abel to run `rustlings`, you need to have rust installed.

⚠️ **DO NOT INSTALL RUST FROM YOUR PACKAGE MANAGER**⚠️

To install rust properly, install its dedicated package manager: [rustup](https://rustup.rs/).

## Running

To run rustlings, clone this repository. You may do this using git, or just downloading it as a zip. The advantage to using git is that if there are updates to this repository, you can automatically get them without losing your progress.

### Install using github (recommended)

```bash
git clone https://github.com/UPB-CS-Rust/rustlings.git
cd rustlings
cargo run
```

Your exercises will be in the `exercises` folder.

To update you just do:

```bash
git pull
```

If you get errors about conflicts, it means an exercise was updated that you already completed. In this situation, it is recommended to ask your lab assistant for help, to ensure you don't lose any progress.

However, you can try to solve it on your own. It is heavily recommended you back up your `exercises` folder before you attempt your update.

To automatically discard your changes and use the new version of the exercise, do `git pull -X theirs`. (Note: uppercase `X`)

### Install using zip (not recommended):

![Press the big `Code` button, then `Download zip`](install_zip.png)

Press the big `Code` button, then `Download zip`.

Afterwards, to run:

```bash
unzip rustlings-main.zip
cd rustlings-main
cargo run
```

To update, you need to manually re-download the archive, and replace the unsolved exercises from the `exercises` folder. Note that all of the exercises you overwrite you will lose progress on. You must manually check for updates on exercises you have already solved.

#### Transfer installation method from zip to git (so you get easier updates)

Simply follow the installation procedure for git, and copy the `exercises` folder you worked on (`rustlings-main/exercises`) to the new rustlings you downloaded via git (`rustlings/exercises`).
