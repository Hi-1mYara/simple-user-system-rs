![Release header](https://github.com/Hi-1mYara/simple-user-system-rs/blob/master/img/ysus_example.png?raw=true)

<h1 align="center">
    <br>
    YSUS
    <br>
</h1>

<h3 align="center">Yara's Simple User System</h3>

<p align="center">
    <a href="https://rust-lang.org/">
        <img alt="[Rust] It's pretty quick!" src="https://img.shields.io/badge/It%27s_pretty_quick!-black?logo=rust&logoColor=white">
    </a>
    <a href="https://www.youtube.com/watch?v=XfELJU1mRMg">
        <img alt="check me out on Youtube" src="https://img.shields.io/badge/Check_me_out_on_Youtube!-red?logo=youtube&logoColor=white">
    </a>
    <a href="https://www.linuxmint.com/">
    <img alt="(Linux mint) It works on my machine" src="https://img.shields.io/badge/%22It_works_on_my_machine%22-86BE43?logo=linuxmint&logoColor=white">
    </a>
</p>

<p align="center">
  <a href="#overview">Overview</a> • 
  <a href="#features">Features</a> • 
  <a href="#planned-features">Planned features</a> • 
  <a href="#installation">Installation</a> • 
  <a href="#reporting-issues">Reporting issues</a> • 
  <a href="#credits">Credits</a>
</p>


## Overview
The Simple User System is a terminal program for creating users and saving them in JSON format, using ratatui for an intuitive TUI. I don't quite now what you would use this for, but do what you want with it.

## Features
- User creation features including:
  - Username, email and UUID
  - Ability to make a user an admin
  - Activity state
  - Deletion of entered user information, leaving the uuid intact
- Saving of users
  - Saves created users to JSON format using serde_json
  - Import users from file and add to it
- TUI
  - Intuitive tui based on ratatui
  - Current action indicator and key notes
  - It looks nice

## Use
To run the program, run the following command
```bash
ysus
```
When you exit the program now, what you have inputted will be displayed in the terminal like this:
```
{
  "1000": {
    "active": true,
    "username": "test",
    "email": "test@example.com",
    "uuid": 1000,
    "admin": true
  }
}
```
To save to file, just add the output file like this:
```bash
ysus > output.json
```

### Tested terminals
- Ptyxis
- GNOME terminal

If you use this in another terminal and it works, open an issue and I will add it to the list.

## Planned features
- [ ] Command line arguments
- [ ] Actual security?
- [ ] Checking for empty fields during user creation
- [ ] Choice between command line arguments and TUI

## Installation
If you want to install this program, you can choose to install it via cargo or building it yourself from this repository

If you do not have Rust installed, follow the installation guide [here](https://doc.rust-lang.org/book/ch01-01-installation.html).

**Important note: this has not been tested on Microsoft Windows and all instructions hereafter are for Linux only**

### Install with cargo
```bash
cargo install simple-user-system
```

### Building from source
```bash
# Clone the repository
# run this command in a directory of your choosing
git clone https://github.com/Hi-1mYara/simple-user-system-rs
cd simple-user-system-rs/

# Build the project
# the executable will be found in target/release/ysus
cargo build --release
```
If you wish to execute the program from the command line by its name alone, put it in $HOME/.cargo/bin

## Reporting issues
When you see a problem in the code, run into bugs or the README/license is wrong, please open an issue. Include in your report the steps to reproduce the errors and your operating system. If you think there is something else about the configuration of your system that may be causing the problem, include that too.  

**Reporting guidelines**
- Be civil and respectful
- Keep it to the problem at hand
  - Only comment on the technical details
  - Keep any personal details out of the issue, for both your privacy and for reducing clutter in your report

## Credits
- Hi_1mYara (as in, me)
- The Rust Programming Language ([The Book](https://doc.rust-lang.org/book/title-page.html))
  - the primary learning material provided by Rust itself

- Inspiration
  - [DocJade](https://github.com/DocJade)
    - The madman who made a filesystem ([Fluster](https://github.com/DocJade/fluster_rs)) to run Factorio from floppy disks
    - a nice example of Rust in practice ([do check him out on youtube too](https://www.youtube.com/@DocJade))
  - [Dysk](https://dystroy.org/dysk/) ([GitHub](https://github.com/Canop/dysk))
    - Inspired me to learn Rust in the first place, i wanted to create projects like it.
