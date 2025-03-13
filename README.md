# Rust CLI Binary with SQLite
Rust CLI Binary that Connects to SQLite Database
[![Build binary release](https://github.com/adlerviton/Rust_SQLite_CLI_Binary/actions/workflows/release.yml/badge.svg)](https://github.com/adlerviton/Rust_SQLite_CLI_Binary/actions/workflows/release.yml)
[![Build](https://github.com/adlerviton/Rust_SQLite_CLI_Binary/actions/workflows/Build.yml/badge.svg)](https://github.com/adlerviton/Rust_SQLite_CLI_Binary/actions/workflows/Build.yml)

Using Rust I created a command line tool which performs CRUD operations on a SQL Database. This creates a Rust binary file in Github Actions to easily distibute thee optimized version of this rust code system. I used Github Copilot and also performed code linting, formatting, and testing. 

### Contents:
- .github/
  - workflows/
  - release.yml
  - clippy.yml
- .gitignore
- src/
  - lib.rs
  - main.rs
- Cargo.toml
- Cargo.lock
- SPX_Data.db
- SPX.csv
- Makefile
─ requirements.txt
─ README.md

The github workflows completes all formatting linting, and testing requirements. 

**Requires rust programming environment: Ensure you have Rust installed on your system.**

### Dependencies
- rusqlite: For SQLite database interaction.
- csv: To work with CSV files.
- clap: for function operations. 

To install the Rust dependencies, they are added to Cargo.toml file the command line "cargo build" is run.

**Building the Binary** 
bash cargo build --release or make release

### Rust Source Code 

All rust source code is contained in the src folder. It consists of two files: lib.rs and main.rs. Lib.rs holds the functions and the tests for the code. Main.rs calls all the functions in the lib.rs and records time taken to complete the operation. 

### SQLite Database: CRUD Operations

  ##### CREATE 
  If the database is not already created, create it using the create_db function from lib.rs. We called this database 'SPX_Data.db'
  ![image](https://github.com/adlerviton/Rust_SQLite_CLI_Binary/blob/main/Images_Rust_Sqlitee/Create_table.png)
  
  ##### READ 
  We can access or read the database by performing the command: "cargo run -- --query 'SELECT * FROM SXP_info'"
  ![image](https://github.com/adlerviton/Rust_SQLite_CLI_Binary/blob/main/Images_Rust_Sqlitee/Read_table.png)
  
  ##### UPDATE
  I changed the dates in the table to only contain the year instead of the full date with day and month. I then re-read the table to see changes.
  ![image](https://github.com/adlerviton/Rust_SQLite_CLI_Binary/blob/main/Images_Rust_Sqlitee/Update_table.png)
  
  ##### DELETE
  Drop Table query deletes the table from the database.
  ![image](https://github.com/adlerviton/Rust_SQLite_CLI_Binary/blob/main/Images_Rust_Sqlitee/Delete_table.png)
  
### Use of GitHub Copilot:

I mostly used Github Copilot to correct rust issues and convert from python. I also used it to create simple tests for my rust functions. I am not particularly well-versed in rust yet, so copilot was very helpful in error troubleshooting.

### Optimized Rust Binary: 

I also created a release executable rust binary called sql runner. This optimized all the functionality in the program and created a binary that can be easily distributed for anyone to use. I created this by running 'make release', which includes all the required dependencies and builds the rust project. 

![image](https://github.com/adlerviton/Rust_SQLite_CLI_Binary/blob/main/Images_Rust_Sqlitee/Make_binary.png)

## Demo Video:

https://youtu.be/zjh7wWrc2E0

