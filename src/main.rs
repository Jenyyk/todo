use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = get_persistent_storage();
    if let Ok(lines) = buffer_file(file) {
        for line in lines {
            match line {
                Ok(line_content) => println!("{}", line_content),
                Err(error) => panic!("{}", error),
            }
        }
    }
}

fn get_persistent_storage() -> File {
    let file_result = File::open("./todoData.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(..) => match File::create("./todoData.txt") {
            Ok(created_file) => created_file,
            Err(error) => panic!("{}", error),
        },
    };
    file
}

fn buffer_file(file_input: File) -> io::Result<io::Lines<io::BufReader<File>>> {
    Ok(io::BufReader::new(file_input).lines())
}
