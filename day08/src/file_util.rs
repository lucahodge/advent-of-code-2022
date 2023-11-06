use std::fs;

pub fn file_to_height_grid (file_name : String) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_name).unwrap();
    let lines : Vec<String> = contents.lines().map(|x| x.to_string()).collect();

    let mut height_grid : Vec<Vec<i32>> = vec!();
    for line in lines {
        height_grid.push(line.chars().map(|x| (x as i32 - '0' as i32)).collect());
    }
    return height_grid;
}

