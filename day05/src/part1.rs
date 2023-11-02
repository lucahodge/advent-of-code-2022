// mod util;
// use util;
use super::util;

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

pub fn part1 () {
    let lines = util::file_to_strings("files/crate_stack.txt".to_string());
    // let lines = file_to_strings("files/crate_stack_test.txt".to_string());
    let (diagram_lines, move_lines) = util::split_on_empty(lines);

    //create the stacks
    let mut stacks = util::diagram_to_stacks(diagram_lines);
    // println!("{}", stacks.len());
    // println!("{:?}", stacks);

    //create the lists of operations
    let moves = util::parse_moves(move_lines);
    // println!("{}", moves.len());

    //operate on stacks
    perform_all_moves(&mut stacks, moves);
    // println!("Final Stack: {:?}", stacks);
    
    //get the top list
    println!("Top list 1: {}", util::get_top_crates(&stacks));
}
