
pub fn part1(height_grid : &Vec<Vec<i32>>) {
    let bool_grid = get_bool_grid(height_grid);
    // println!("{:?}", bool_grid);

    let number_of_visible = get_number_of_visible(&bool_grid);
    println!("Number of visible trees: {}", number_of_visible);
}

fn get_number_of_visible(bool_grid : &Vec<Vec<bool>>) -> i32 {
    let mut num = 0;
    for row in bool_grid.iter() {
        for b in row.iter(){
            if *b {
                num += 1;
            }
        }
    }
    return num;
}

fn get_bool_grid(height_grid : &Vec<Vec<i32>>) -> Vec<Vec<bool>>{
    let len1 = height_grid.len();
    let len2 = height_grid[0].len();
    let mut bool_grid = vec![vec![false; len2]; len1];
    for i in 0..len1 {
        {
            let mut prev = -1;
            let mut j = 0;
            while j < len2 {
                if height_grid[i][j] > prev {
                    bool_grid[i][j] = true;
                    prev = height_grid[i][j];
                }
                j+=1;
            }
        }
        {
            let mut prev = -1;
            let mut j = len2;
            while j > 0 {
                j -= 1;
                if height_grid[i][j] > prev {
                    bool_grid[i][j] = true;
                    prev = height_grid[i][j];
                }
            }
        }
    }
    for j in 0..len2 {
        {
            let mut prev = -1;
            let mut i = 0;
            while i < len1 {
                if height_grid[i][j] > prev {
                    bool_grid[i][j] = true;
                    prev = height_grid[i][j];
                }
                i+=1;
            }
        }
        {
            let mut prev = -1;
            let mut i = len1;
            while i > 0 {
                i -= 1;
                if height_grid[i][j] > prev {
                    bool_grid[i][j] = true;
                    prev = height_grid[i][j];
                }
            }
        }
    }
    // for i in 0..char_grid[0].len() {
    // }
    // println!("{:?}", bool_grid);
    return bool_grid;
}
