# Experiments with csv crate

Trying to read and write to csv files using the csv crate documentation.

Crate here: https://crates.io/crates/csv 

Tutorial here: https://docs.rs/csv/1.1.5/csv/tutorial/index.html

Since i just rewrite the code whenever I need i store the some examples here for reference.

## Error handling
A quick example of correct error handling with the ? operator.

```
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
```

