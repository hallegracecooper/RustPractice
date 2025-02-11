use std::collections::HashMap;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u32,
}

impl Book {
    // Constructor-like method to create a new Book instance
    fn new(title: &str, author: &str, year: u32) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            year,
        }
    }

    // Method to display the book details
    fn display(&self) {
        println!("Title: {}\nAuthor: {}\nYear: {}\n", self.title, self.author, self.year);
    }

    // Method to check if the book is a classic (published before 1950)
    fn is_classic(&self) -> bool {
        self.year < 1950
    }
}

fn main() {
    // Step 1: Creating a Vec to store multiple Book objects
    let mut library: Vec<Book> = Vec::new();

    // Step 2: Creating books using the new method and adding them to the Vec
    let book1 = Book::new("The Great Gatsby", "F. Scott Fitzgerald", 1925);
    let book2 = Book::new("1984", "George Orwell", 1949);
    let book3 = Book::new("The Rust Book", "Steve Klabnik & Carol Nichols", 2018);

    library.push(book1);
    library.push(book2);
    library.push(book3);

    // Step 3: Display all books in the library
    println!("Library Contents (using Vec):");
    for book in &library {
        book.display();
    }

    // Step 4: Creating a HashMap to store books with the title as the key
    let mut book_map: HashMap<String, Book> = HashMap::new();

    // Step 5: Adding books to the HashMap
    let book4 = Book::new("Moby Dick", "Herman Melville", 1851);
    let book5 = Book::new("The Catcher in the Rye", "J.D. Salinger", 1951);

    book_map.insert(book4.title.clone(), book4);
    book_map.insert(book5.title.clone(), book5);

    // Step 6: Retrieving a book by title using HashMap
    let book_title = "Moby Dick";
    println!("\nRetrieving book by title '{}':", book_title);
    match book_map.get(book_title) {
        Some(book) => book.display(),
        None => println!("Book not found."),
    }

    // Checking if the books in the map are classics
    println!("\nChecking if books in the map are classics:");
    for (title, book) in &book_map {
        println!("Is '{}' a classic? {}", title, book.is_classic());
    }
}