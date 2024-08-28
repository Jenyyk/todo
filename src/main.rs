use std::fs::File;

fn main() {
    let file_result = File::open("todoData");
    let _file = match file_result {
        Ok(file) => file,
        Err(..) => match File::create("totoData") {
            Ok(created_file) => created_file,
            Err(..) => panic!(),
        },
    };
}
