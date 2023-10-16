///
/// File: lib.rs
/// Author: Steven Kight
/// Date: 2023-09-25
/// Description:
///   This is the lib file for the Sysprog-P1-StevenKight project.
///   This project is a part of the course work for the course CS-3280 at the University of West Georgia.
///   This file contains the functions to read the input files, process data in files, and utilize processed data.
/// 

use std::{fs, io::BufRead, sync::mpsc::Sender};

/// 
/// Read all the directories in the given directory and return a vector of directories.
/// 
/// Arguments
/// ---------
/// * `folder_path` - The path of the folder to read
/// 
/// Returns
/// ---------
/// * `files` - A vector of file names (with path) in the given folder
/// 
pub fn read_directory(folder_path: &str) -> Vec<String> {
    let mut paths_vec: Vec<String> = Vec::new();
    let paths = fs::read_dir(folder_path).unwrap();
    for path in paths {
        let path_name = path.unwrap().path().display().to_string();
        paths_vec.push(path_name);
    }
    paths_vec
}

///
/// This takes the contents of a file and returns the summary of the file as a string.
/// 
/// Arguments
/// ---------
/// * `file_contents` - The contents of the file as a string
/// 
/// Returns
/// ---------
/// * `summary` - The summary of the file as a string
/// 
fn process_file_contents(file_contents: &std::fs::File) -> String {
    let mut summary = String::new();
    
    let lines = std::io::BufReader::new(file_contents).lines();

    let mut name = String::new();
    let mut product = String::new();

    let mut quantity_sum = 0;
    for line in lines.collect::<Vec<_>>() { // TODO: Optimize this loop
        let read_line = line.unwrap();
        let trimmed_line = read_line.trim();

        // Split the line by commas
        let line_split: Vec<&str> = trimmed_line.split(", ").collect();

        // Get the name
        name = line_split[0].to_string();

        // Get the product
        product = line_split[1].to_string();

        // Get the quantity
        let quantity = line_split[2].parse::<i32>().unwrap();
        quantity_sum += quantity;
    }
    
    summary.push_str(&format!("{}, ", name));
    summary.push_str(&format!("{}, ", product));
    summary.push_str(&format!("{}\n", quantity_sum));
    summary
}

///
/// This takes a vector of directories and processes all the files in the directories.
/// 
/// Arguments
/// ---------
/// * `directories` - A vector of directories to process
/// 
/// Returns
/// ---------
/// * `status` - The status of the processing. "OK" if successful, "ERROR" if not.
/// 
pub fn process_input(directories: Vec<String>, tx: Sender<String>) -> String {

    for directory in directories {
        let files: Vec<String> = read_directory(&directory);
        for file_name in files {
            // Ignore the summary file
            if file_name.contains("summary") {
                continue;
            }

            // Open the file
            let file = std::fs::File::open(&file_name).expect("Unable to open file.");

            // Process the file
            let summary = process_file_contents(&file);
            tx.send(summary);
            
            // Close the file
            std::mem::drop(file);
        }
    }

    "OK".to_string()
}
