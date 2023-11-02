// use crate::util;
use super::util;
// mod util;

fn perform_move_2 (stacks : &mut Vec<Vec<char>>, m: (i32,i32,i32)) {
    let mut my_crates = vec!();
    for _ in 0..m.0 {
        my_crates.push(stacks[m.1 as usize].pop().unwrap());
    }
    for my_crate in my_crates.into_iter().rev() {
        stacks[m.2 as usize].push(my_crate);
    }
}

fn perform_all_moves_2 (stacks : &mut Vec<Vec<char>>, moves: Vec<(i32,i32,i32)>) {
    for m in moves {
        perform_move_2(stacks, m);
        // println!("In all moves: {:?}", stacks);
    }
}

pub fn part2 () {
    let lines = util::file_to_strings("files/crate_stack.txt".to_string());
    // let lines = file_to_strings("files/crate_stack_test.txt".to_string());
    let (diagram_lines, move_lines) = util::split_on_empty(lines);

    //create the stacks
    let mut stacks = util::diagram_to_stacks(diagram_lines);

    //create the lists of operations
    let moves = util::parse_moves(move_lines);

    //operate on stacks
    perform_all_moves_2(&mut stacks, moves);
    
    //get the top list
    println!("Top list 2: {}", util::get_top_crates(&stacks));
}
