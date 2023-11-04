use std::fs;

pub fn file_to_strings (file_name : String) -> Vec<String> {
    let contents = fs::read_to_string(file_name).expect("should read from file");
    return contents.lines().map(|x| x.to_string()).collect();
}

pub fn split_on_empty (lines : Vec<String>) -> (Vec<String>, Vec<String>) {
    for i in 0..lines.len() {
        let line = &lines[i];
        if line.len() == 0 {
            // let (a,b) = lines.split_at(i);
            // return (Vec::from(a), Vec::from(b));
            // let (a,b) = (lines[0..i], lines[i+1..]);
            // let (a,b) = (lines.iter().take(i).collect(), lines.iter().skip(i+1).collect());
            let a = lines.iter().take(i).map(|x| x.clone()).collect();
            let b = lines.iter().skip(i+1).map(|x| x.clone()).collect();
            return (a, b);
            
            // return (Vec::from(a), Vec::from(b));
        }
    }
    return (lines, vec!());
}

pub fn split_on_char(s: String, c: char) -> Vec<String> {
    let mut words : Vec<String> = vec!();
    let mut prev_i = 0;
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == c {
            // return (s.chars().take(i).collect(), s.chars().skip(i+1).collect());
            words.push(s.chars().skip(prev_i).take(i-prev_i).collect());
            prev_i = i + 1;
        }
    }
    words.push(s.chars().skip(prev_i).collect());

    return words;

}                             
