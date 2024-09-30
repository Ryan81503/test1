use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};

#[derive(Debug)]
struct Car {
    make: String,
    model: String,
    year: u16,
}

fn main() -> io::Result<()> {
    // Get user input for the car's details
    let mut make = String::new();
    let mut model = String::new();
    let mut year_str = String::new();

    println!("Enter the make of your car:");
    io::stdin().read_line(&mut make).expect("Failed to read input");
    println!("Enter the model of your car:");
    io::stdin().read_line(&mut model).expect("Failed to read input");
    println!("Enter the year of your car:");
    io::stdin().read_line(&mut year_str).expect("Failed to read input");

    // Convert year input to u16
    let year: u16 = year_str.trim().parse().expect("Year must be a number");

    // Create a Car struct
    let car = Car {
        make: make.trim().to_string(),
        model: model.trim().to_string(),
        year,
    };

    // Write the struct to a file
    let mut file = OpenOptions::new().create(true).append(true).open("user_info.txt")?;
    writeln!(file, "Make: {}", car.make)?;
    writeln!(file, "Model: {}", car.model)?;
    writeln!(file, "Year: {}", car.year)?;

    println!("Car details have been saved to user_info.txt");

    // Read the file and print its content
    let file = File::open("user_info.txt")?;
    let reader = BufReader::new(file);

    println!("\nContents of user_info.txt:");
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}