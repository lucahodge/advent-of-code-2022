use std::fs;

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn file_to_moves (file_name : String) -> Vec<(Move, i32)> {
    let mut moves = vec!();
    let lines : Vec<String> = fs::read_to_string(file_name).unwrap().lines().map(|x| x.to_string()).collect();
    for line in lines {
        let chars : Vec<char> = line.chars().collect();
        let num : i32 = chars[2] as i32 - '0' as i32;
        let move_option = match chars[0] {
            'U' => Some(Move::Up),
            'D' => Some(Move::Down),
            'L' => Some(Move::Left),
            'R' => Some(Move::Right),
            _ => None,
        };
        match move_option {
            Some(m) => {
                moves.push((m, num));
            },
            None => {},
        };
    }
    return moves;
}

fn num_tail_locations (moves : Vec<(Move, i32)>) -> i32 {
    let mut head = (0,0);
    let mut tail = (0,0);
    for (m, n) in moves {
        for _ in 0..n {
            head = move_head(head, &m);
            println!("{:?}", head);
        }
    }
    return 0;
}

fn move_head (head: (i32, i32), m : &Move) -> (i32, i32) {
    match m {
        Move::Up => {
            return (head.0, head.1 + 1);
        },
        Move::Down => {
            return (head.0, head.1 - 1);
        },
        Move::Right => {
            return (head.0 + 1, head.1);
        },
        Move::Left => {
            return (head.0 - 1, head.1);
        },
    }
}

fn main() {
    println!("Hello, world!");
    let moves = file_to_moves(String::from("files/head_movements_test.txt"));
    println!("{:?}", moves);
    let _ = num_tail_locations(moves);
}
