use std::fs;
use std::io::Error;

fn main() -> Result<(), Error>{
    let contents = fs::read_to_string("../input")?;
    print!("{}", contents);
    Ok(())
}
