# Rust System Monitor
## Description
RSM (Rust System Monitor) is a resource monitor designed to be straightforward, easy to use and lightweight. It shows system information, CPU & memory usage, active processes list and stats for disks
and networks, all in a clean and minimalistic UI.

## Features
* **Lightweight & fast:** Written in Rust with minimal overhead.  
* **Real-time resource monitor:** Provides nicely-presented statistics about OS, CPU, memory, processes, disks and networks - refreshed every 1 second.  
* **Readability:** Focuses on clean and readable UI.  

## Installation
### Using Cargo
If you have Rust installed, you can install it using Cargo:  
```bash  
cargo install rust-system-monitor
```
### Linux
You can download and run the installation script by pasting this command:
```bash
curl -fsSL https://raw.githubusercontent.com/r3nzgmd/rsm/main/install.sh | sh
```
Then, open the program by typing:
```bash
rsm
```
Note: make sure you have the folder `/usr/bin` added to PATH.

## Compilation
If you want to compile the code yourself on your machine, you will need `rustc v1.97.1` or later. Then, run this command to clone the repository and build from source code:
```bash
curl -fsSL https://raw.githubusercontent.com/r3nzgmd/rsm/main/compile.sh | sh
```

## Screenshots
![alt](src)
