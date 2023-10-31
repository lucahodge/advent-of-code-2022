use std::fs;

fn split_into_lines (text: String) -> Vec<String> {
    let mut v : Vec<String> = vec!();
    let mut previous_index = 0;
    for i in 0..text.len() {
        if text.chars().nth(i).unwrap() == '\n' {
            v.push(text[previous_index..i].to_string());
            previous_index = i+1;
        }
    }
    return v;
}

fn split_on_char(s: String, c: char) -> (String, String) {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == c {
            return (s.chars().take(i).collect(), s.chars().skip(i+1).collect());
        }
    }
    return (s, String::new());
    
}

fn touple_to_i32 (t: (String, String)) -> (i32,i32) {
    return (t.0.parse().unwrap(), t.1.parse().unwrap());
}

fn parse_to_tuples (s: String) -> ((i32, i32), (i32, i32)) {
    let (half1, half2) = split_on_char(s, ',');
    let a = split_on_char(half1, '-');
    let b = split_on_char(half2, '-');
    // return ((a.0.parse().unwrap(), a.1.parse().unwrap()), (b.0.parse().unwrap(), b.1.parse().unwrap()));
    return (touple_to_i32(a), touple_to_i32(b));
}

fn part1(){
    let contents = fs::read_to_string("files/pairs_list.txt").expect("to be able to read from file");
    let lines = split_into_lines(contents);
    let mut num_fully_contained = 0;
    for line in lines {
        let (a, b) = parse_to_tuples(line);
        // println!("{:?} {:?}", a, b);
        if (a.0 <= b.0 && a.1 >= b.1) || (a.0 >= b.0 && a.1 <= b.1) {
            num_fully_contained += 1;
        }
    }
    println!("Part 1: {}", num_fully_contained);
}

fn part2(){
    let contents = fs::read_to_string("files/pairs_list.txt").expect("to be able to read from file");
    let lines = split_into_lines(contents);
    let mut num_overlapped = 0;
    for line in lines {
        let (a, b) = parse_to_tuples(line);
        if !(a.1 < b.0 || b.1 < a.0){
            num_overlapped += 1;
        }
    }
    println!("Part 2: {}", num_overlapped);
}

fn main() {
    part1();
    part2();
}
