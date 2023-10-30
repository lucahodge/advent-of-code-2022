use std::fs;

fn split_into_lines (text: String) -> Vec<String> {
    let mut v : Vec<String> = vec!();
    // let mut s = String::new();
    let mut previous_index = 0;
    for i in 0..text.len() {
        if text.chars().nth(i).unwrap() == '\n' {
            v.push(text[previous_index..i].to_string());
            previous_index = i+1;
        }
    }
    return v;
}

fn most_calories (filename : String) -> i32 {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let v = split_into_lines(contents);
    let mut most_count = 0;
    let mut curr_count = 0;
    for e in v {
        match e.as_str() {
            "" => {
                if curr_count > most_count {
                    most_count = curr_count;
                }
                curr_count = 0;
            }
            _ => {
                let cal_num : i32 = e.parse().unwrap();
                curr_count += cal_num;
            }
        }
    }
    if curr_count > most_count {
        most_count = curr_count;
    }
    return most_count;
}

fn main() {
    let ret = most_calories(String::from("src/calories-list.txt"));
    println!("{}", ret);
    
}
