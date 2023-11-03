mod util;
mod parts;

fn main() {
    // let part1_result = part1::part1(util::file_to_strings(String::from("files/signal_test.txt"))[0].clone());
    let part1_result = parts::part1(util::file_to_strings(String::from("files/signal.txt"))[0].clone());
    println!("part1: {}", part1_result);
    let part2_result = parts::part2(util::file_to_strings(String::from("files/signal.txt"))[0].clone());
    println!("part2: {}", part2_result);
}
