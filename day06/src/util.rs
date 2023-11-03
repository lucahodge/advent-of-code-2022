use std::fs;

pub fn file_to_strings (file_name : String) -> Vec<String> {
    let contents = fs::read_to_string(file_name).expect("should read from file");
    return contents.lines().map(|x| x.to_string()).collect();
}
