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

#[derive(Debug)]
enum RPS {
    Rock,
    Paper, 
    Scissors,
}

fn decrypt_opponent_move (character: char) -> Option<RPS> {
    match character {
        'A' => {return Some(RPS::Rock);}
        'B' => {return Some(RPS::Paper);}
        'C' => {return Some(RPS::Scissors);}
        _ => {return None;}
    }
}
fn decrypt_my_move (character: char) -> Option<RPS> {
    match character {
        'X' => {return Some(RPS::Rock);}
        'Y' => {return Some(RPS::Paper);}
        'Z' => {return Some(RPS::Scissors);}
        _ => {return None;}
    }
}
fn correct_decrypt_my_move (character: char, opponent_move: &RPS) -> Option<RPS> {
    return match (character, opponent_move) {
        ('X', RPS::Rock) => Some(RPS::Scissors),
        ('X', RPS::Paper) => Some(RPS::Rock),
        ('X', RPS::Scissors) => Some(RPS::Paper),
        ('Y', RPS::Rock) => Some(RPS::Rock),
        ('Y', RPS::Paper) => Some(RPS::Paper),
        ('Y', RPS::Scissors) => Some(RPS::Scissors),
        ('Z', RPS::Rock) => Some(RPS::Paper),
        ('Z', RPS::Paper) => Some(RPS::Scissors),
        ('Z', RPS::Scissors) => Some(RPS::Rock),
        _ => None,
    }
}

fn get_score (opponent_move: RPS, my_move: RPS) -> i32 {
    let mut score : i32 = match my_move {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
    score = score + match ( opponent_move, my_move) {
        (RPS::Rock, RPS::Rock) => 3,
        (RPS::Rock, RPS::Scissors) => 0,
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Paper, RPS::Rock) => 0,
        (RPS::Paper, RPS::Scissors) => 6,
        (RPS::Paper, RPS::Paper) => 3,
        (RPS::Scissors, RPS::Rock) => 6,
        (RPS::Scissors, RPS::Scissors) => 3,
        (RPS::Scissors, RPS::Paper) => 0,
    };
    return score;
}

fn main() {
    let file_contents = fs::read_to_string("src/strategy-guide.txt").expect("Should be able to read from file");
    let lines = split_into_lines(file_contents);

    let mut total_score : i32 = 0;
    for line in lines {
        let opponent_move = decrypt_opponent_move(line.chars().nth(0).unwrap()).unwrap();
        // let my_move = decrypt_my_move(line.chars().nth(2).unwrap()).unwrap();
        let my_move = correct_decrypt_my_move(line.chars().nth(2).unwrap(), &opponent_move).unwrap();
        let score = get_score(opponent_move, my_move);
        total_score += score;
    }
    println!("total score: {}", total_score);
}
