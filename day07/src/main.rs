mod file_util;
mod directory;

fn main() {
    let lines = file_util::file_to_strings(String::from("files/commands.txt"));
    // let lines = file_util::file_to_strings(String::from("files/commands_test.txt"));
    directory::part1(lines.clone());
    directory::part2(lines.clone());
}
