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
    let mut visited : Vec<(usize, usize)> = vec!();
    let mut pq : PriorityQueue<i32, (usize, usize)> = PriorityQueue::new();
    pq.push(0, start);
    while(pq.len() > 0) {
        let curr = pq.pop().unwrap().1;
        visited.push(curr);
    }
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
