
pub fn part2 (height_grid : &Vec<Vec<i32>>) {
    let max_scenic = get_max_scenic(height_grid);
    println!("Heighest scenic score: {}", max_scenic);
}

fn get_max_scenic (height_grid : &Vec<Vec<i32>>) -> i32 {
    let mut max_scenic = 0;
    let len1 = height_grid.len();
    let len2 = height_grid[0].len();
    for i in 0..len1 {
        for j in 0..len2 {
            let curr_senic = calculate_scenic(i, j, height_grid);
            // println!("{} {} {}", i, j, curr_senic);
            if curr_senic > max_scenic {
                max_scenic = curr_senic;
            }
        }
    }
    return max_scenic;
}

fn calculate_scenic (i : usize, j: usize, height_grid : &Vec<Vec<i32>>) -> i32 {
    let mut scenic : i32 = 1;
    let len1 = height_grid.len();
    let len2 = height_grid[0].len();

    let my_tree_height = height_grid[i][j];

    {
        let mut x = i+1;
        let mut blocked = false;
        while x < len1 && !blocked {
            if height_grid[x][j] >= my_tree_height{
                blocked = true;
            }
            x += 1;
        }
        let multiplyer = x - (i+1);
        scenic *= multiplyer as i32;
    }
    {
        let mut x = i;
        let mut blocked = false;
        while x > 0 && !blocked {
            x -= 1;
            if height_grid[x][j] >= my_tree_height{
                blocked = true;
            }
        }
        let multiplyer = i - x;
        scenic *= multiplyer as i32;
    }
    {
        let mut x = j+1;
        let mut blocked = false;
        while x < len2 && !blocked {
            if height_grid[i][x] >= my_tree_height{
                blocked = true;
            }
            x += 1;
        }
        let multiplyer = x - (j+1);
        scenic *= multiplyer as i32;
    }
    {
        let mut x = j;
        let mut blocked = false;
        while x > 0 && !blocked {
            x -= 1;
            if height_grid[i][x] >= my_tree_height{
                blocked = true;
            }
        }
        let multiplyer = j - x;
        scenic *= multiplyer as i32;
    }
    return scenic;
}
