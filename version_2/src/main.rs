///
/// File: main.rs
/// Author: Steven Kight
/// Date: 2023-09-25
/// Description:
///    This is the main file for the Sysprog-P1-StevenKight project.
///    This project is a part of the course work for the course CS-3280 at the University of West Georgia.
/// 

use version_2::read_directory;
use version_2::process_input;

use std::process;
use std::thread;
use std::sync::mpsc;
use std::io::Write;

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

    // Read the directory and get the list of files
    let branches = read_directory(&data_folder);

    // Split the branches vector into 4 vectors and setup channels
    let num_threads = 3;
    let mut receivers: Vec<mpsc::Receiver<String>> = Vec::new();

    // Split the branches vector into num_threads vectors
    let branch_chunks = branches.chunks(branches.len() / num_threads).map(|x| x.to_vec()).collect::<Vec<_>>();

    let start = std::time::Instant::now(); // start the timer

    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for chunk in &branch_chunks {
        let branch_chunk = chunk.clone();
        let (tx, rx) = mpsc::channel();
        receivers.push(rx);

        // Spin up another thread
        children.push(thread::spawn(move || {
            let response = process_input(branch_chunk, tx);
            println!("Thread finished with response: {}", response);
        }));
    }

    for rx in receivers {
        for _i in &branch_chunks[0] {
            let summary = rx.recv().unwrap();
            write_to_summary_file(summary, &output_folder);
        }
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    let end = start.elapsed(); // stop the timer

    println!("\nTotal time taken: {:?}", end); // print the total time taken
    println!("Phew! I am done."); // print the message to indicate processing of all files are done
}

///
/// This takes a summary string and writes it to the summary file.
/// 
/// Arguments
/// ---------
/// * `summary` - The summary string to write to the file
/// * `output_folder` - The output folder path
/// 
fn write_to_summary_file(summary: String, output_folder: &str) {
    let output_file = format!("{}/weekly_sales_summary.txt", output_folder);
    if !std::path::Path::new(&output_file).exists() {
        std::fs::File::create(&output_file).expect("Unable to create output file."); 
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&output_file)
        .unwrap();

    file.write_all(summary.as_bytes()).expect("Unable to write data");

    std::mem::drop(file);
}
