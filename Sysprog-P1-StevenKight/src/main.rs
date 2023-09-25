///
/// File: main.rs
/// Author: Steven Kight
/// Date: 2023-09-25
/// Description:
///    This is the main file for the Sysprog-P1-StevenKight project.
///    This project is a part of the course work for the course CS-3280 at the University of West Georgia.
/// 

///
/// Accept, validate and parse the data folder name (with path) from the command line argument.
/// Check if the output folder exists in the data folder - if not, create it.
/// 
/// ### Returns
/// * `input_folder` - The input folder path
/// * `output_folder` - The output folder path
/// 
fn process_args() -> (String, String) {
    let data_folder = std::env::args().nth(1).expect("Please provide a data folder.");

    // Validate if the data folder exists
    println!("Data folder given: {}", data_folder);
    if !std::path::Path::new(&data_folder).exists() {
        panic!("Data folder does not exist.");
    }

    // Validate if the input folder exists
    let input_folder = format!("{}/input", data_folder);
    if !std::path::Path::new(&input_folder).exists() {
        panic!("Input folder does not exist.");
    }

    // Create the output folder if it does not exist
    let output_folder = format!("{}/output", data_folder);
    if !std::path::Path::new(&output_folder).exists() {
        std::fs::create_dir(&output_folder).expect("Unable to create output folder."); 
    }

    println!("- Input folder: {}", input_folder);
    println!("- Output folder: {}", output_folder);
    // return the input and output folder paths
    (input_folder, output_folder)
}

fn main() {
    // Accept, validate and parse the data folder name (with path) from the command line argument
    process_args();

    let start = std::time::Instant::now(); // start the timer

    // TODO: Call the file input function in lib.rs and pass the list of folders (with path) for all the branches.

    let end = start.elapsed(); // stop the timer

    println!("\nTotal time taken: {:?}", end); // print the total time taken
    println!("Phew! I am done."); // print the message to indicate processing of all files are done
}
