use std::io;
use std::process;
use std::error::Error;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    
}

fn run() -> Result<(), Box<dyn Error>> {
    // Create a CSV parser that reads data from stdin.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Loop over each record.
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}