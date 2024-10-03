use std::process::ExitCode;
use std::fs::{File, OpenOptions};
use std::io::{stdin, self, Write, BufRead};
use std::collections::BTreeMap;
use colored::*;

fn main() -> ExitCode {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();
    loop {
        let file = get_persistent_storage("./todo_data.txt");
        let lines = match buffer_file(&file) {
            Ok(file_contents) => file_contents,
            Err(error) => panic!("{}", error),
        };

        let mut line_map: BTreeMap<u16, String> = BTreeMap::new();
        for (index, line) in lines.enumerate() {
            if let Ok(line_content) = line {
                line_map.insert(index.try_into().unwrap(), line_content);
            }
        }

        for (num, line) in &line_map {
            let mut split_line = line.split("¦¦");
            let line_text = split_line.next().unwrap_or("");
            let color = split_line.next().unwrap_or("");
            let red: u8;
            let green: u8;
            let blue: u8;
            if color == "" {
                (red, green, blue) = (255, 255, 255);
            } else {
                let mut split_colors = color.split_whitespace();
                (red, green, blue) = (split_colors.next().unwrap().parse::<u8>().unwrap(), split_colors.next().unwrap().parse::<u8>().unwrap(), split_colors.next().unwrap().parse::<u8>().unwrap());
            }
            println!("{} {} {}", colored(100, 250, 255, ">"), colored(249, 241, 165, &num.to_string()), colored(red, green, blue, &line_text));
        }

        let mut input = String::new();
        store_user_inputs(&mut input);
        if input.trim() == "exit" {
            return ExitCode::SUCCESS;
        }
        let mut iter = input.trim().split_whitespace();
        match iter.next() {
            Some("add") => todo_add(iter.collect::<Vec<&str>>().join(" "), &mut line_map),
            Some("del") => todo_del(iter.map(|s| s.parse::<u16>().unwrap()).collect(), &mut line_map),
            Some("mov") => todo_mov(iter.map(|s| s.parse::<u16>().unwrap()).collect(), &mut line_map),
            _ => { println!("{}", colored(250, 60, 60, "wrong input")); println!() },
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
    print!("$ ");
    io::stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(input).expect("incorrect input");
    println!();
}

fn colored(r: u8, g: u8, b: u8, text: &str) -> String {
    // format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text)
    text.truecolor(r, g, b).to_string()
}

fn todo_add(to_add: String, lines: &mut BTreeMap<u16, String>) {
    let mut result = String::new();
    for (_num, line) in lines {
        result.push_str(line);
        result.push('\n');
    }
    result.push_str(&to_add);
    result.push('\n');
    let mut file = get_persistent_storage("./todo_data.txt");
    if let Err(error) = file.write_all(result.as_bytes()) { panic!("{}", error); }
    let _ = file.flush();
}

fn todo_del(to_del: Vec<u16>, lines: &mut BTreeMap<u16, String>) {
    use io::{Seek, SeekFrom};
    let mut result = String::new();
    for (num, line) in lines {
        if !to_del.contains(num) {
            result.push_str(line);
            result.push('\n');
        }
    }
    let mut file = get_persistent_storage("./todo_data.txt");
    if let Err(error) = file.seek(SeekFrom::Start(0)) { panic!("{}", error); }
    if let Err(error) = file.write_all(result.as_bytes()) { panic!("{}", error); }
    let _ = file.flush();
    let current_position = file.seek(SeekFrom::Current(0)).expect("failed deleting");
    file.set_len(current_position).expect("failed deleting");
}

fn todo_mov(to_mov: Vec<u16>, lines: &mut BTreeMap<u16, String>) {
    swap_values(lines, *to_mov.get(0).unwrap(), *to_mov.get(1).unwrap());
    let mut result = String::new();
    for (_num, line) in lines {
        result.push_str(line);
        result.push('\n');
    }
    let mut file = get_persistent_storage("./todo_data.txt");
    if let Err(error) = file.write_all(result.as_bytes()) { panic!("{}", error); }
    let _ = file.flush();
}

fn swap_values<K: Ord + Copy, V>(map: &mut BTreeMap<K, V>, key1: K, key2: K) {
    if let (Some(value1), Some(value2)) = (map.remove(&key1), map.remove(&key2)) {
        map.insert(key1, value2);
        map.insert(key2, value1);
    }
}
