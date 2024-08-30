use std::process::ExitCode;
use std::fs::{File, OpenOptions};
use std::io::{stdin, self, Write, BufRead};
use std::collections::BTreeMap;

fn main() -> ExitCode {
    loop {
        let file = get_persistent_storage("./todo_data.txt");
        let lines = match buffer_file(&file) {
            Ok(file_contents) => file_contents,
            Err(error) => panic!("{}", error),
        };

        let mut line_map: BTreeMap<u8, String> = BTreeMap::new();
        for (index, line) in lines.enumerate() {
            if let Ok(line_content) = line {
                line_map.insert(index.try_into().unwrap(), line_content);
            }
        }

        for (num, line) in &line_map {
            println!("> {num} {line}")
        }

        let mut input = String::new();
        store_user_inputs(&mut input);
        if input.trim() == "exit" {
            return ExitCode::SUCCESS;
        }
        let mut iter = input.trim().split_whitespace();
        match iter.next() {
            Some("add") => todo_add(iter.collect::<Vec<&str>>().join(" "), &mut line_map),
            _ => todo!(),
        }

    }
}

fn get_persistent_storage(path: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap()
}

fn buffer_file(file_input: &File) -> io::Result<io::Lines<io::BufReader<&File>>> {
    Ok(io::BufReader::new(file_input).lines())
}

fn store_user_inputs(input: &mut String) {
    println!();
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(input).expect("incorrect input");
    println!();
}

fn todo_add(to_add: String, lines: &mut BTreeMap<u8, String>) {
    let mut result = String::new();
    for (_num, line) in lines {
        result.push_str(line);
        result.push('\n');
    }
    result.push_str(&to_add);
    result.push('\n');
    let mut file = get_persistent_storage("./todo_data.txt");
    if let Err(error) = file.write_all(result.as_bytes()) {
        panic!("{}", error);
    }
    let _ = file.flush();
}
