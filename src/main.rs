use std::fs::File;
use std::io::Write;
use std::io::Result;

// Function that writes data to a file
fn write_to_file(data: String) -> Result<()> {
    let mut file = File::create("output.txt")?; // Creates the file
    file.write_all(data.as_bytes())?; // Writes the data
    Ok(())
}

fn main() {
    let test_data = String::from("Hello, world!");
    // Call the `write_to_file` function
    let result = write_to_file(test_data.clone());

    match result {
        Ok(_) => println!("Data written successfully!"),
        Err(e) => println!("Error writing to file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_write_to_file() {
        let test_data = String::from("Hello, test!");

        // Call the function
        let result = write_to_file(test_data.clone());

        // Check if the result is Ok
        assert!(result.is_ok());

        // Now let's read the file to ensure the data was written correctly
        let mut file = File::open("output.txt").expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");

        // Assert that the file contents match the test data
        assert_eq!(contents, test_data);
    }

    #[test]
    fn test_empty_write() {
        let empty_data = String::new();

        // Call the function with empty data
        let result = write_to_file(empty_data.clone());

        // Check if the result is Ok
        assert!(result.is_ok());

        // Verify that the file is empty
        let mut file = File::open("output.txt").expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");

        // Assert that the file contents are empty
        assert_eq!(contents, empty_data);
    }
}