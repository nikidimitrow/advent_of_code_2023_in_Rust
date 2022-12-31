use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the input file
    let input = match fs::read_to_string("../input"){
        Ok(input) => input,
        Err(err) => return Err(Box::new(err)),
    };

    //Check if the input file contained only digit
    let mut only_numbers = true;
    for ch in input.chars() {
        if !ch.is_numeric() {
            only_numbers = false;
            break;
        }
    }

    // Split the file into lines
    let lines: Vec<&str> = input.lines().collect();

    // Initialize variables to keep track of the current index and the current sum
    let mut index = 0;
    let mut temp_index = 0;
    let mut sum = 0;
    let mut temp_sum = 0;

    // Iterate over the lines in the file
    for line in lines {
        // If the line is empty, print the index and sum and reset them
        if line.is_empty() {
            //println!("Index: {} Sum: {}", index, sum);
            index += 1;
            sum = 0;
            
        } else {
            // Otherwise, parse the number and add it to the sum
            let number: i32 = line.parse()?;
            sum += number;
            //Collected the biggest sum at the current moment and the index
            if !line.is_empty() {
                if temp_sum < sum {
                    temp_sum = sum;
                    temp_index = index;
                }
            }
        }
    }

    // Print the final sum and index
    println!("Sum: {}, Index {}", temp_sum, temp_index);
    // Final answer is 67450 calories

    Ok(())
}