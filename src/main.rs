use std::process::ExitCode;
use std::fs::{File, OpenOptions};
use std::io::{stdin, self, Write, BufRead};
use std::collections::BTreeMap;
use colored::*;

fn main() -> ExitCode {
    // important for coloring in windows
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();
    loop {
        // reads file
        let file = get_persistent_storage("./todo_data.txt");
        let lines = match buffer_file(&file) {
            Ok(file_contents) => file_contents,
            Err(error) => panic!("{}", error),
        };

        // loads file into a Binary Tree Map
        let mut line_map: BTreeMap<u16, String> = BTreeMap::new();
        for (index, line) in lines.enumerate() {
            if let Ok(line_content) = line {
                line_map.insert(index.try_into().unwrap(), line_content);
            }
        }

        // loops through each line
        for (num, line) in &line_map {
            // Splits between todo content and color
            let mut split_line = line.split("¦¦");
            let line_text = split_line.next().unwrap_or("");
            // translates colors, defaults to white
            let color = split_line.next().unwrap_or("");
            let red: u8;
            let green: u8;
            let blue: u8;
            if color == "" {
                (red, green, blue) = (255, 255, 255);
            } else {
                let mut split_colors = color.split_whitespace();
                red = split_colors.next().unwrap_or("").parse::<u8>().unwrap_or(255);
                green = split_colors.next().unwrap_or("").parse::<u8>().unwrap_or(255);
                blue = split_colors.next().unwrap_or("").parse::<u8>().unwrap_or(255);
            }
            // prints line with color
            println!("{} {} {}", colored(100, 250, 255, ">"), colored(249, 241, 165, &num.to_string()), colored(red, green, blue, &line_text));
        }

        // gets user input
        let mut input = String::new();
        store_user_inputs(&mut input);
        if input.trim() == "exit" {
            return ExitCode::SUCCESS;
        }
        // clears screen
        clear_screen();
        // puts input into iterator by splitting by whitespace
        let mut iter = input.trim().split_whitespace();
        // Input checking
        match iter.next() {
            // first item of iterator dictates command
            Some("add") => todo_add(iter.collect::<Vec<&str>>().join(" "), &mut line_map), // joins the rest of the iterator into a string and adds it
            Some("del") => todo_del(iter.map(|s| s.parse::<u16>().unwrap_or(65535)).collect(), &mut line_map), // collects the rest of the iterator and deletes it all
            Some("move") => todo_move(iter.map(|s| s.parse::<u16>().unwrap_or(65535)).collect(), &mut line_map), // collects the rest of the iterator and swaps based on first two items in iterator
            Some("color") => todo_color(iter.next().unwrap().parse::<u16>().unwrap_or(65535), iter.collect::<Vec<&str>>().join(" "), &mut line_map), // next item in iterator is which input to color, rest of iterator is the color
            Some("help") => todo_print_help(), // prints help
            Some("bea") => { println!("{}", colored(255, 20, 220, "cutýsek")); println!() },
            _ => { println!("{}", colored(250, 60, 60, "wrong input")); println!() }, // prints red error to signify wrong input
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

fn clear_screen() {
    #[cfg(unix)]
    std::process::Command::new("clear").status().expect("failed clearing terminal screen");
    #[cfg(windows)]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("failed clearing terminal screen");
    }
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
    file.seek(SeekFrom::Start(0)).expect("failed writing while deleting");
    file.write_all(result.as_bytes()).expect("failed writing while deleting");
    let _ = file.flush();
    let current_position = file.seek(SeekFrom::Current(0)).expect("failed deleting");
    file.set_len(current_position).expect("failed deleting");
}

fn todo_move(to_move: Vec<u16>, lines: &mut BTreeMap<u16, String>) {
    swap_values(lines, *to_move.get(0).unwrap(), *to_move.get(1).unwrap());
    let mut result = String::new();
    for (_num, line) in lines {
        result.push_str(line);
        result.push('\n');
    }
    let mut file = get_persistent_storage("./todo_data.txt");
    file.write_all(result.as_bytes()).expect("failed writing while swapping values");
    let _ = file.flush();
}
fn swap_values<K: Ord + Copy, V>(map: &mut BTreeMap<K, V>, key1: K, key2: K) {
    if let (Some(value1), Some(value2)) = (map.remove(&key1), map.remove(&key2)) {
        map.insert(key1, value2);
        map.insert(key2, value1);
    }
}

fn todo_color(to_color: u16, color: String, lines: &mut BTreeMap<u16, String>) {
    use io::{Seek, SeekFrom};
    let mut result = String::new();
    for (num, line) in lines {
        if *num == to_color {
            let mut split_line = line.split("¦¦");
            result.push_str(split_line.next().expect("failed to split color data"));
            result.push_str("¦¦");
            result.push_str(&color);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    let mut file = get_persistent_storage("./todo_data.txt");
    file.seek(SeekFrom::Start(0)).expect("failed writing while deleting");
    file.write_all(result.as_bytes()).expect("failed writing while deleting");
    let _ = file.flush();
    let current_position = file.seek(SeekFrom::Current(0)).expect("failed deleting");
    file.set_len(current_position).expect("failed deleting");
}

fn todo_print_help() {
    println!("- {} prints this menu", colored(160, 160, 160, "help"));
    println!("- {} adds a task  ", colored(160, 160, 160, "add"));
    println!("- {} deletes tasks, based on index, can take multiple arguments", colored(160, 160, 160, "del ~..."));
    println!("- {} swaps two tasks based on index", colored(160, 160, 160, "move ~ ~"));
    println!("- {} colors a task based on the index and color input", colored(160, 160, 160, "color ~ r g b"));
    println!();
    println!();
}
