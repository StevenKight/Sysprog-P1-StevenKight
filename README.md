# Sysprog-P1-StevenKight

## Rust Code


## Cargo
The Rust project was built using Cargo. The Cargo.toml files can be found at the following paths:
- [./version_1/Cargo.toml](./version_1/Cargo.toml)
- [./version_2/Cargo.toml](./version_2/Cargo.toml)

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
2. cd into the wanted versions `/src/` directory
   1. Version 1 is the single threaded version found in the [./version_1/src](./version_1/src) directory
   2. Version 2 is the multithreaded version found in the [./version_2/src](./version_2/src) directory
3. run `cargo run <data directory path>` to run the program with the data directory path as the argument (e.g. `./data` for the included test data directory)

## Improvements from version 1 to version 2:

Through the use of threading, a significant performance increase was achieved going from between 1.5-2ms to ~1ms. This was achieved by splitting the data into 4 chunks and processing each chunk in a separate thread. The threads were then joined and the results were written to the output file. The code for this can be found in the `version 2` directory.