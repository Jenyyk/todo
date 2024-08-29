use std::fs::File;
use std::io::{stdin, stdout, Write, self, BufRead};
use std::collections::HashMap;

fn main() {
    loop {
        let file = get_persistent_storage();
        let lines = match buffer_file(file) {
            Ok(file_contents) => file_contents,
            Err(error) => panic!("{}", error),
        };

        let mut line_map: HashMap<u8, String> = HashMap::new();
        for (index, line) in lines.enumerate() {
            if let Ok(line_content) = line {
                line_map.insert(index.try_into().unwrap(), line_content);
            }
        }

        let mut input = String::new();
        stdin().read_line(&mut input).expect("incorrect input");
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
