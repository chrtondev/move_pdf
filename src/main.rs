// Import necessary modules from the standard library and the walkdir crate.
use std::env; // For accessing environment variables, such as the home directory.
use std::fs; // For file system operations like creating directories and renaming (moving) files.
use std::path::PathBuf;
use walkdir::WalkDir; // From the walkdir crate, to easily walk through directory trees. // For handling file paths in a cross-platform way.

fn main() {
    // Retrieve the path to the home directory.
    // This uses the HOME environment variable, common on Unix-like systems, including Mac.
    let home_dir = env::var("HOME").expect("\nCould not find HOME directory");

    // Define the source directory where the .pdf files are initially located.
    // This typically would be the Downloads directory in the user's home directory.
    let source_dir = PathBuf::from(format!("{}/Downloads", home_dir));

    // Define the target directory where the .pdf files will be moved to.
    // In this example, it's a PDFs folder inside the Documents directory in the user's home directory.
    // Feel free to change "PDFs" to any folder name that suits your needs.
    let target_dir = PathBuf::from(format!("{}/Documents/PDFs", home_dir));

    // Check if the target directory exists; if not, create it and all necessary parent directories.
    // This ensures that the program doesn't crash if the specified target directory doesn't exist.
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).expect("Failed to create target directory");
    }

    // Walk through the source directory, including all subdirectories.
    // .into_iter() transforms the WalkDir into an Iterator, allowing us to loop through each entry.
    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok()) // Use filter_map to ignore any errors and unwrap the Ok values.
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "pdf"))
    // Filter only .pdf files.
    {
        // Define the target path for each .pdf file, maintaining the original file name.
        let target_path = target_dir.join(entry.file_name());

        // Attempt to move (rename) each .pdf file to the target directory.
        // If the operation fails, panic with a custom error message showing the problematic file path.
        fs::rename(entry.path(), &target_path)
            .unwrap_or_else(|_| panic!("Failed to move {:?}", entry.path()));

        // Print a confirmation message showing the source and target paths for each moved file.
        println!("Moved {:?} to {:?}", entry.path(), target_path);
    }
}
