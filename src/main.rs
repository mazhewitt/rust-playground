
use std::fs::read_to_string;
use std::io;
use std::env;
use std::ops::Deref;
use mockall::automock;

struct MyFile {
    filename: String,
}

#[automock]
trait MyFileLoader {
    fn load_file(&self) -> io::Result<String>;
}

impl MyFileLoader for MyFile {
    fn load_file(&self) -> io::Result<String> {
        read_to_string(&self.filename)
    }
}

fn load_file_from_string(file_loader :&dyn MyFileLoader) -> io::Result<String> {
    file_loader.load_file()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let my_file_loader = MyFile {
        filename: filename.to_string(),
    };
    match load_file_from_string(&my_file_loader) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Error loading file: {}", e),
    }
}

//


#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    #[test]
    fn test_load_file_from_string() {
        let filename = "test.txt";
        let contents = "Hello, world!";
        let mut mock = MockMyFileLoader::new();
        mock.expect_load_file().returning(|| Ok(contents.to_string()));

        assert_eq!(load_file_from_string(&mock).unwrap(), contents);
    }
}




