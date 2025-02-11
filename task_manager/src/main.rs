use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u32,
}

impl Book {
    fn new(title: &str, author: &str, year: u32) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            year,
        }
    }

    fn display(&self) {
        println!("Title: {}\nAuthor: {}\nYear: {}\n", self.title, self.author, self.year);
    }
}

fn main() -> io::Result<()> {
    // Step 1: Read from a file
    let path = Path::new("books.txt");
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e);
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => println!("File contents:\n{}", contents),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return Err(e);
        }
    }

    // Step 2: Create some Book instances to write to a new file
    let book1 = Book::new("The Great Gatsby", "F. Scott Fitzgerald", 1925);
    let book2 = Book::new("1984", "George Orwell", 1949);
    let book3 = Book::new("The Catcher in the Rye", "J.D. Salinger", 1951);

    // Step 3: Write to a new file
    let write_path = Path::new("new_books.txt");
    let mut new_file = match OpenOptions::new().write(true).create(true).open(write_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file for writing: {}", e);
            return Err(e);
        }
    };

    let books = vec![book1, book2, book3];
    for book in books {
        match writeln!(new_file, "{} by {} ({}).", book.title, book.author, book.year) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error writing to file: {}", e);
                return Err(e);
            }
        }
    }

    // Step 4: Print out the file contents again to ensure writing was successful
    println!("\nSuccessfully wrote to the new file. Here is the content:");

    let mut new_file_contents = String::new();
    let mut new_file = match File::open(write_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reopening the new file: {}", e);
            return Err(e);
        }
    };
    
    match new_file.read_to_string(&mut new_file_contents) {
        Ok(_) => println!("{}", new_file_contents),
        Err(e) => {
            eprintln!("Error reading the new file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}