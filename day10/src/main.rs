
#[derive(Debug)]
enum Instruction{
    Noop,
    Addx(i32),
}

struct Register {
    value : i32,
}

impl Register {
    fn new(starting_value : i32) -> Register {
        return Register{value : starting_value};
    }

    fn cycle(&self, instruction : Instruction) -> i32 {
        return 0;

    }
}

fn file_to_instuctions(file_name : String) -> Vec<Instruction> {
    let contents = std::fs::read_to_string(file_name).unwrap();
    let lines : Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    let mut instructions : Vec<Instruction> = vec!();
    for line in lines {
        let word : Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        match word[0].as_str() {
            "noop" => instructions.push(Instruction::Noop),
            "addx" => instructions.push(Instruction::Addx(word[1].parse().unwrap())),
            _ => {},
        };
    }
    return instructions;
}

fn main() {
    println!("Hello, world!");
    let instructions = file_to_instuctions(String::from("files/instructions_test.txt"));
    println!("{:?}", instructions);

    
}
