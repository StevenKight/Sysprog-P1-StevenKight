# Sysprog-P1-StevenKight

## Rust Code
The rust code for this project can be found in the following files:
- [main.rs](./Sysprog-P1-StevenKight/src/main.rs)
- [lib.rs](./Sysprog-P1-StevenKight/src/lib.rs)

## Cargo
The Rust project was built using Cargo. The Cargo.toml file can be found at the following path:
- [Cargo.toml](./Sysprog-P1-StevenKight/Cargo.toml)

## Objectives
Specific objectives of the assignment are listed below - 
1. Learn what type of programming can benefit from multi-core multithreaded programs.
2. Learn how to divide data for data parallelization to gain performance
3. Learn to create multithreaded programs in RUST
4. Learn to handle resource sharing in a multithreaded program
5. Learn File I/O in RUST

## Full Description:
Full description can be found in the following pdf:
- [Project1_sysprog_Fall23.pdf](https://github.com/StevenKight/Sysprog-P1-StevenKight/files/12716300/Project1_sysprog_Fall23.pdf)

## How to run:
1. Clone the repository
2. Navigate to the Sysprog-P1-StevenKight directory
3. Follow the instructions below for the version you want to run
4. rustc main.rs && ./main <output_file_path> (e.g. `rustc main.rs && ./main ../data`)

### To run version 1:
3. cd into the `version 1` directory

### To run version 2:
3. cd into the `version 2` directory

## Improvements from version 1 to version 2:

Through the use of threading, a significant performance increase was achieved going from just under 2ms to around 0.5ms. This was achieved by splitting the data into 4 chunks and processing each chunk in a separate thread. The threads were then joined and the results were written to the output file. The code for this can be found in the `version 2` directory.