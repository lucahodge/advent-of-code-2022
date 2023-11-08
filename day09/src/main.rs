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
        // let num : i32 = chars[2] as i32 - '0' as i32;
        let num_str : String = line.chars().skip(2).collect();
        let num : i32 = num_str.parse().unwrap();
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

fn num_tail_locations (moves : &Vec<(Move, i32)>) -> i32 {
    let mut tail_visited : Vec<(i32, i32)> = vec!();
    let mut head = (0,0);
    let mut tail = (0,0);
    tail_visited.push(tail);
    for (m, n) in moves {
        for _ in 0..*n {
            move_head(&mut head, m);
            move_tail(&mut tail, &head);
            match tail_visited.iter().find(|x| x.0 == tail.0 && x.1 == tail.1) {
                None => {
                    tail_visited.push(tail);
                },
                Some(_) => {},
            }
            // println!("{:?} {:?}", head, tail);
        }
    }
    return tail_visited.len() as i32;
}

fn move_head (head: &mut (i32, i32), m : &Move) {
    match m {
        Move::Up => {
            head.1 = head.1 + 1;
        },
        Move::Down => {
            head.1 = head.1 - 1;
        },
        Move::Right => {
            head.0 = head.0 + 1;
        },
        Move::Left => {
            head.0 = head.0 - 1;
        },
    }
}

fn move_tail (tail: &mut (i32,i32), head : &(i32,i32)) {
    // should move diag if possible
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    // if x_diff.abs() <= 1 && y_diff.abs() <=1 {
    //     return;
    // }
    // if x_diff == 0 {
    //     let step = y_diff/y_diff.abs();
    //     tail.1 = tail.1 + step;
    // }
    // else if y_diff == 0 {
    //     let step = x_diff/x_diff.abs();
    //     tail.0 = tail.0 + step;
    // }
    // else {
    //     let step = y_diff/y_diff.abs();
    //     tail.1 = tail.1 + step;
    //     let step = x_diff/x_diff.abs();
    //     tail.0 = tail.0 + step;
    // }


    if x_diff.abs() <= 1 && y_diff.abs() <=1 {
        return;
    }

    if x_diff != 0 {
        let step = x_diff/x_diff.abs();
        tail.0 = tail.0 + step;
    }
    if y_diff != 0 {
        let step = y_diff/y_diff.abs();
        tail.1 = tail.1 + step;
    }
}

fn num_tail_locations_long (moves : &Vec<(Move, i32)>, rope_len : usize) -> i32 {
    let mut tail_visited : Vec<(i32, i32)> = vec!();
    // let mut head = (0,0);
    // let mut tail = (0,0);
    let mut rope : Vec<(i32, i32)> = vec![(0,0);rope_len];
    tail_visited.push(*rope.last().unwrap());
    for (m, n) in moves {
        for _ in 0..*n {
            move_head(&mut rope[0], m);
            for i in 1..rope.len(){
                let (front, back) = rope.split_at_mut(i);
                // let (front, back) = rope.split_at_mut(i);
                move_tail(&mut back[0], &front.last().unwrap());
                // let h = &rope[i-1];
                // move_tail(&mut rope[i], h);
            }
            match tail_visited.iter().find(|x| x.0 == rope.last().unwrap().0 && x.1 == rope.last().unwrap().1) {
                None => {
                    tail_visited.push(*rope.last().unwrap());
                },
                Some(_) => {},
            }
            // println!("{:?} {:?}", rope[0], rope.last().unwrap());
        }
    }
    return tail_visited.len() as i32;
}

fn main() {
    let moves = file_to_moves(String::from("files/head_movements.txt"));
    // let moves = file_to_moves(String::from("files/head_movements_test.txt"));
    // println!("{:?}", moves);
    let part1 = num_tail_locations(&moves);
    println!("Part 1: {}", part1);
    let part2 = num_tail_locations_long(&moves, 10);
    println!("Part 2: {}", part2);
}
