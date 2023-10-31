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

fn get_shared_char (a:String, b:String) -> char {
    let mut va : Vec<char> = a.chars().collect();
    let mut vb : Vec<char> = b.chars().collect();
    va.sort();
    vb.sort();
    let mut ia = 0;
    let mut ib = 0;
    while ia < va.len() && ib < vb.len() {
        let ca = va[ia];
        let cb = vb[ib];
        if ca == cb {
            return ca;
        }
        else if ca < cb {
            ia += 1;
        }
        else {
            ib += 1;
        }
    }
    return '_';
}

fn get_shared_char_triplet (t: (String, String, String)) -> char {
    let mut va : Vec<char> = t.0.chars().collect();
    let mut vb : Vec<char> = t.1.chars().collect();
    let mut vc : Vec<char> = t.2.chars().collect();
    va.sort();
    vb.sort();
    vc.sort();
    let mut ia = 0;
    let mut ib = 0;
    let mut ic = 0;
    while ia < va.len() && ib < vb.len() {
        let ca = va[ia];
        let cb = vb[ib];
        if ca == cb {
            while ic < vc.len() && vc[ic] < ca {
                ic +=1;
            }
            if ic >= vc.len() {
                return '_';
            }
            if vc[ic] == ca {
                return ca;
            }
            ia += 1;
            ib += 1;
        }
        else if ca < cb {
            ia += 1;
        }
        else {
            ib += 1;
        }
    }
    return '_';
}

fn get_priority (c :char) -> i32 {
    match c {
        'a'..='z' => {
            c as i32 - 'a' as i32 + 1
        },
        'A'..='Z' => {
            c as i32 - 'A' as i32 + 27
        },
        _ => 0,
    }
}

fn part1(){
    let contents = fs::read_to_string("src/rucksack-list.txt")
        .expect("should be able to read");
    // let contents = fs::read_to_string("src/rucksack-list-example.txt")
    //     .expect("should be able to read");
    let lines = split_into_lines(contents);

    let mut total_priority = 0;
    for line in lines {
        let compartment1 : String = line.chars().take(line.len()/2).collect(); //line[0..line.len()/2];
        let compartment2 : String = line.chars().skip(line.len()/2).collect(); //line[line.len()/2..line.len()];

        let shared_char = get_shared_char(compartment1, compartment2);

        let priority = get_priority(shared_char);
        // println!("line:{}, shared:{}, priority:{}", line, shared_char, priority);

        total_priority += priority;
    }

    println!("Part 1 sum of priorities: {}", total_priority);
}

fn part2(){
    let contents = fs::read_to_string("src/rucksack-list.txt")
        .expect("should be able to read");
    let lines = split_into_lines(contents);

    let mut triplets : Vec<(String, String, String)>= vec!();
    for i in (0..lines.len()).step_by(3) {
        triplets.push((lines[i].clone(), lines[i+1].clone(), lines[i+2].clone()));
    }

    let mut total_priority = 0;
    for triplet in triplets {
        let shared_char = get_shared_char_triplet(triplet);
        let priority = get_priority(shared_char);
        total_priority += priority;
    }

    println!("Part 2 sum of priorities: {}", total_priority);
}

fn main() {
    part1();
    part2();
}
