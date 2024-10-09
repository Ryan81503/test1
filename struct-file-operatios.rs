use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // Create or truncate the file for writing
    let mut file = File::create(filename).expect("Unable to create file");

    for book in books {
        // Format each book's details as "title,author,year\n"
        let line = format!("{},{},{}\n", book.title, book.author, book.year);
        file.write_all(line.as_bytes()).expect("Unable to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.expect("Unable to read line");
        let parts: Vec<&str> = line.trim().split(',').collect();

        // Ensure that each line has exactly 3 parts
        if parts.len() != 3 {
            eprintln!("Skipping malformed line: {}", line);
            continue;
        }

        let title = parts[0].to_string();
        let author = parts[1].to_string();
        let year: u16 = match parts[2].parse() {
            Ok(y) => y,
            Err(_) => {
                eprintln!("Invalid year in line: {}", line);
                continue;
            }
        };

        books.push(Book { title, author, year });
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "The Rust Programming Language".to_string(), author: "Steve Klabnik and Carol Nichols".to_string(), year: 2018 },
    ];

    let filename = "books.txt";

    // Save books to the file
    save_books(&books, filename);
    println!("Books saved to file '{}'.", filename);

    // Load books from the file
    let loaded_books = load_books(filename);
    println!("\nLoaded books:");
    for book in loaded_books {
        println!("\"{}\" by {}, published in {}", book.title, book.author, book.year);
    }
}