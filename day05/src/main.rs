use std::fs;

fn file_to_strings (file_name : String) -> Vec<String> {
    let contents = fs::read_to_string(file_name).expect("should read from file");
    return contents.lines().map(|x| x.to_string()).collect();
}

fn split_on_empty (lines : Vec<String>) -> (Vec<String>, Vec<String>) {
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

fn split_on_char(s: String, c: char) -> Vec<String> {
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

fn diagram_to_stacks (lines: Vec<String>) -> Vec<Vec<char>> {
    let mut simplified_lines = vec!();
    for line in &lines {
        let mut simplified_line = String::new();
        for i in (0..line.len()).step_by(4) {
            // print!("{}", line.chars().nth(i+1).unwrap());
            simplified_line.push(line.chars().nth(i+1).unwrap());
        }
        simplified_lines.push(simplified_line);
    }

    let mut stacks : Vec<Vec<char>> = vec!();
    let numbers = simplified_lines.pop().unwrap();
    let number_of_stacks = numbers.len();
    for _ in 0..number_of_stacks {
        stacks.push(vec!());
    }

    for i in (0..simplified_lines.len()).rev() {
        // let row = &simplified_lines[i];
        for (i, c) in simplified_lines[i].chars().enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    return stacks;
}

fn parse_moves (lines: Vec<String>) -> Vec<(i32, i32, i32)> {
    let mut moves = vec!();
    for line in lines {
        let words = split_on_char(line, ' ');
        // println!("{} {} {}", words[1], words[3], words[5]);
        moves.push((words[1].parse::<i32>().unwrap(), words[3].parse::<i32>().unwrap()-1, words[5].parse::<i32>().unwrap()-1))
    }
    return moves;
}

fn perform_move (stacks : &mut Vec<Vec<char>>, m: (i32,i32,i32)) {
    for _ in 0..m.0 {
        // println!("Made one move!");
        let my_crate = stacks[m.1 as usize].pop().unwrap();
        stacks[m.2 as usize].push(my_crate);
    }

}

fn perform_all_moves (stacks : &mut Vec<Vec<char>>, moves: Vec<(i32,i32,i32)>) {
    for m in moves {
        perform_move(stacks, m);
        // println!("In all moves: {:?}", stacks);
    }
}

fn get_top_crates (stacks : &Vec<Vec<char>>) -> String {
    let mut s : String = String::new();
    for stack in stacks {
        s.push(stack.last().unwrap().clone());
    }
    return s;
}

fn part1 () {
    let lines = file_to_strings("files/crate_stack.txt".to_string());
    // let lines = file_to_strings("files/crate_stack_test.txt".to_string());
    let (diagram_lines, move_lines) = split_on_empty(lines);

    //create the stacks
    let mut stacks = diagram_to_stacks(diagram_lines);
    // println!("{}", stacks.len());
    // println!("{:?}", stacks);

    //create the lists of operations
    let moves = parse_moves(move_lines);
    // println!("{}", moves.len());

    //operate on stacks
    perform_all_moves(&mut stacks, moves);
    // println!("Final Stack: {:?}", stacks);
    
    //get the top list
    println!("Top list : {}", get_top_crates(&stacks));
}

fn main() {
    part1();
}
