use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the input file
    let input = fs::read_to_string("../input")?;

    // Split the file into lines
    let lines: Vec<&str> = input.lines().collect();

    // Initialize variables to keep track of the current index and the current sum
    let mut index = 0;
    let mut sum = 0;

    // Iterate over the lines in the file
    for line in lines {
        // If the line is empty, print the index and sum and reset them
        if line.is_empty() {
            println!("Index: {} Sum: {}", index, sum);
            index += 1;
            sum = 0;
        } else {
            // Otherwise, parse the number and add it to the sum
            let number: i32 = line.parse()?;
            sum += number;
        }
    }

    // Print the final index and sum
    println!("Index: {} Sum: {}", index, sum);

    Ok(())
}