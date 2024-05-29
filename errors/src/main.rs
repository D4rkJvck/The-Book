use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // -> Result<(), Box<dyn Error>> {
    match read_file("hello.txt") {
        Ok(content) => {
            println!("\nFile content:\n{:?}", content);

            let last = last_char_of_first_line(&content);
            println!("\nLast Character of first line: {:?}", last);
        }
        Err(error) => panic!("\nProblem reading content:\n{:?}", error),
    };
}

////////////////////////////////////////////////////////////////////////

fn read_file(path: &str) -> Result<String, io::Error> {
    println!("\nReading file...");

    let mut content = String::new();

    open_file(path)?.read_to_string(&mut content)?;
    Ok(content)
}

//----------------------------------------------------------

fn open_file(path: &str) -> Result<File, io::Error> {
    println!("\nOpening file...");

    match File::open(path) {
        Ok(file) => {
            println!("\n{:#?}", file);
            Ok(file)
        }
        Err(error) => {
            println!("\nProblem opening file:\n{:?}", error);
            match error.kind() {
                ErrorKind::NotFound => {
                    create_file(path)?;
                    open_file(path)
                }
                _ => Err(error),
            }
        }
    }
}

//----------------------------------------------------------

fn create_file(path: &str) -> Result<File, io::Error> {
    println!("\nCreating file...");

    match File::create(path) {
        Ok(file) => {
            println!("\n{:#?}", file);
            Ok(file)
        }
        Err(error) => {
            println!("\nProblem creating file:\n{:?}", error);
            Err(error)
        }
    }
}

//----------------------------------------------------------

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
