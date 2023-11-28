use priority_queue::PriorityQueue;

fn file_to_height_map(file_name : String) -> (Vec<Vec<char>>, (usize,usize), (usize,usize)){
    let lines : Vec<String> = std::fs::read_to_string(file_name).unwrap().lines().map(|x| x.to_string()).collect();
    let mut map : Vec<Vec<char>> = vec!();
    for line in lines {
        let chars : Vec<char> = line.chars().collect();
        map.push(chars);
    }

    let mut start = (0,0);
    let mut end = (0,0);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = map[y][x];
            match c {
                'S' => {
                    start = (y,x);
                    map[y][x] = 'a'
                },
                'E' => {
                    end = (y,x);
                    map[y][x] = 'z'
                },
                _ => {},
            }
        }
    }

    return (map, start, end);
}

fn generate_distance_map(height_map : Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {
    // using negative values so shortest distance has highest priority
    let mut visited : Vec<(usize, usize)> = vec!();
    let mut pq : PriorityQueue<(usize, usize), i32> = PriorityQueue::new();
    pq.push(start, 0);
    while pq.len() > 0 {
        let (curr_loc, curr_dist) = pq.pop().unwrap();
        visited.push(curr_loc);

        // println!("CURR {} {:?}, {:?}", curr_dist, curr_loc, pq);
        if curr_loc == end {
            println!("found it {}", curr_dist);
            return;
        }

        let mut next_to : Vec<(usize,usize)> = vec!();
        if curr_loc.0 > 0 {
            next_to.push((curr_loc.0-1, curr_loc.1));
        }
        if curr_loc.1 > 0 {
            next_to.push((curr_loc.0, curr_loc.1-1));
        }
        if curr_loc.0 < height_map.len() {
            next_to.push((curr_loc.0+1, curr_loc.1));
        }
        if curr_loc.1 < height_map[0].len() {
            next_to.push((curr_loc.0, curr_loc.1+1));
        }
        for next in next_to {
            if !visited.contains(&next) {
                pq.push(next, curr_dist-1);
            }
        }
    }
    println!("___");
    for e in visited {
        println!("{:?}", e);
    }
}

fn main() {
    let (map, start, end) = file_to_height_map(String::from("./files/height_map_test.txt"));

    // for row in map {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    generate_distance_map(map, start, end);
}
