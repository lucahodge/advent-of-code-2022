mod file_util;
mod part1;
mod part2;

fn main() {
    // let height_grid = file_util::file_to_height_grid(String::from("files/trees_test.txt"));
    let height_grid = file_util::file_to_height_grid(String::from("files/trees.txt"));
    // println!("{:?}", height_grid);
    part1::part1(&height_grid);
    part2::part2(&height_grid);
}


// fn is_visible(i : usize, j : usize, char_grid : &Vec<Vec<char>>) -> bool {
//     return true;
// }
