

pub fn part1(signal : String) -> i32 {
    for i in 3..signal.len() {
        let mut duplicate = false;
        for j in (i-3)..=i {
            for k in (i-3)..j {
                // println!("{} {} {}", i, j, k);
                // println!("{} {}", signal.chars().nth(j).unwrap(), signal.chars().nth(k).unwrap());
                if signal.chars().nth(j).unwrap() == signal.chars().nth(k).unwrap() {
                    duplicate = true;
                }
            }
        }
        if !duplicate {
            return (i + 1) as i32;
        }
    }
    return 0;
}


pub fn part2(signal : String) -> i32 {
    //TODO make this faster
    for i in 13..signal.len() {
        let mut duplicate = false;
        for j in (i-13)..=i {
            for k in (i-13)..j {
                // println!("{} {} {}", i, j, k);
                // println!("{} {}", signal.chars().nth(j).unwrap(), signal.chars().nth(k).unwrap());
                if signal.chars().nth(j).unwrap() == signal.chars().nth(k).unwrap() {
                    duplicate = true;
                }
            }
        }
        if !duplicate {
            return (i + 1) as i32;
        }
    }
    return 0;
}
