//Logan Burley
//12/8/2023
//Bible Lookup App
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use textwrap::fill;

fn main() -> io::Result<()> {
    let mut keep_searching: bool = true;

    let referenced_verse_file_path: String = String::from("Referenced_verse.txt");

    let mut book_abbreviations: HashMap<String, String> = HashMap::new();

    let bible_abrv = File::open("C:/Users/ljbur/OneDrive/CU/PLS_L6/src/Bible_Abbreviations.csv").expect("Could not open file");
    let reader_csv = BufReader::new(bible_abrv);

    for line in reader_csv.lines() {
        let line = line.unwrap();
        let mut values = line.split(",");
        book_abbreviations.insert(values.next().unwrap().to_string().to_uppercase(),
                                     values.next().unwrap().to_string().to_uppercase());
    }

    println!("Enter the reference of the verse you want to retrieve");

    while keep_searching {
        let mut search_again: String = String::new();
    
        //book, chapter and verse variables
        let mut book: String = String::new(); 
        let mut chapter: String = String::new();
        let mut verse: String = String::new();

        let mut book_title: String = String::from("THE BOOK OF ");

        //_ tells compiler it is purposefully unused
        let mut _chapter_title: String = String::new(); 

        //get book 
        print!("Enter Book: ");
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin().read_line(&mut book).expect("Failed to Readline");
        book = book.trim().to_uppercase().to_string(); //clean input
        
        //check if book is in csv
        if let Some(value) = book_abbreviations.get(&book) {
            book = value.to_string();
        }

        book_title.push_str(&book); //append book to book title
        
        //get chapter
        print!("Enter Chapter: ");
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin().read_line(&mut chapter).expect("Failed to Readline"); 
        chapter = chapter.trim().to_string();

        if book_title == "THE BOOK OF PSALMS" {
            _chapter_title = "PSALM ".to_string();
            _chapter_title.push_str(&chapter);
        }
        else {
            _chapter_title = "CHAPTER ".to_string();
            _chapter_title.push_str(&chapter);
        }

        //get verse
        print!("Enter Verse: ");
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin().read_line(&mut verse).expect("Failed to Readline");
        verse = verse.trim().to_string() + " "; //trim but then add one space to the end

        //book, chapter, and verse found bools
        let mut book_found: bool = false;
        let mut chapter_found: bool = false;
        let mut verse_found: bool = false;

        //open bible file
        let bible = File::open("C:/Users/ljbur/OneDrive/CU/PLS_L6/src/Bible.txt")?;
        let reader = BufReader::new(&bible);

        for line in reader.lines() {
            //search through the file
            if let Ok(mut content) = line {
                //stop searching outside bounds of book
                if book_found && content.contains("THE BOOK OF ") {
                    break;
                }
                //stop searching outside bounds of chapter
                if chapter_found && content.contains("CHAPTER ") {
                    break;
                }
                //find book
                if content.contains(&book_title) {
                    book_found = true;
                } 
                //find chapter
                if book_found && content.contains(&_chapter_title) {
                    chapter_found = true;
                }
                //find verse
                if chapter_found && content.contains(&verse) {
                    verse_found = true;

                    //insert colon after verse number
                    if let Some(idx) = content.find(' ') {
                        content.insert(idx, ':')
                    }

                    content.insert_str(0, " "); // add space
                    content.insert_str(0, &chapter); // add chapter
                    content.insert_str(0, " "); // add space
                    content.insert_str(0, &book); // add book

                    //write verse to file
                    let mut verse_file = OpenOptions::new().append(true).create(true).open(&referenced_verse_file_path)
                                                                                           .expect("Could not open file");
                    writeln!(verse_file, "{}", content)?;
  
                    //pretty print
                    let pretty_print_reference = fill(&content, 80);
                    println!("\n{}", pretty_print_reference);
                    break;
                }
            } 
        }    
        if !book_found {
            println!("\nBook '{}' does not exist", book.trim());
        }     
        else if book_found && !chapter_found {
            println!("\nChapter '{}' in the book '{}' does not exist", chapter, book);
        }        
        else if book_found && chapter_found && !verse_found {
            println!("\nVerse '{}' in chapter '{}' of book '{}' does not exist", verse, chapter, book);
        }
        //ask if user wants to search again
        print!("\nWould you like to search again? (Y/N): ");
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin().read_line(&mut search_again).expect("Failed to Readline");
        search_again = search_again.trim().to_string();
        if search_again.to_lowercase() != "y" {
            keep_searching = false;
        }
     }
    Ok(())
}
