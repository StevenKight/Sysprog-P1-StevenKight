///
/// File: main.rs
/// Author: Steven Kight
/// Date: 2023-09-25
/// Description:
///    This is the main file for the Sysprog-P1-StevenKight project.
///    This project is a part of the course work for the course CS-3280 at the University of West Georgia.
/// 

use version_1::read_directory;
use version_1::process_input;

use std::process;

///
/// Accept, validate and parse the data folder name (with path) from the command line argument.
/// Check if the output folder exists in the data folder - if not, create it.
/// 
/// Pre-conditions
/// ---------
/// * The data folder name (with path) is passed as a command line argument.
/// * The data folder exists.
/// 
/// Returns
/// ---------
/// * `input_folder` - The input folder path
/// * `output_folder` - The output folder path
/// 
fn process_args() -> (String, String) {
    let args = std::env::args().nth(1);

    // Validate arguments
    if args.is_none() {
        eprintln!("Please provide a data folder in the format: `cargo run <data_folder>`");
        process::exit(1);
    }
    let data_folder = args.unwrap();

    // Validate if the data folder exists
    println!("Data folder given: {}", data_folder);
    if !std::path::Path::new(&data_folder).exists() {
        eprintln!("Data folder given does not exist.");
        process::exit(1);
    }

    // Create the output folder if it does not exist
    let output_folder = format!("{}/weekly_summary", data_folder);
    if !std::path::Path::new(&output_folder).exists() {
        std::fs::create_dir(&output_folder).expect("Unable to create output folder."); 
    }
    else {
        // If the output folder exists, check if the weekly_summary.txt file exists and delete it if it does.
        let output_file = format!("{}/weekly_sales_summary.txt", output_folder);
        if std::path::Path::new(&output_file).exists() {
            std::fs::remove_file(output_file).expect("Unable to delete weekly_summary.txt file.");
        }
    }

    // return the main and output folder paths
    (data_folder, output_folder)
}

fn main() {
    // Accept, validate and parse the data folder name (with path) from the command line argument
    let (data_folder, output_folder) = process_args();

    let branches = read_directory(&data_folder);

    let start = std::time::Instant::now(); // start the timer

    let response = process_input(branches, &output_folder);
    println!("{}", response);

    let end = start.elapsed(); // stop the timer

    println!("\nTotal time taken: {:?}", end); // print the total time taken
    println!("Phew! I am done."); // print the message to indicate processing of all files are done
}
