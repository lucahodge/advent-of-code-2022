// need to have it be executing cycles

#[derive(Debug)]
enum Instruction{
    Noop,
    Addx(i32),
}

struct Register {
    value : i32,
    executing_instructions : Vec<(i32, Instruction)>,
}

impl Register {
    fn new(starting_value : i32) -> Register {
        return Register{value : starting_value, executing_instructions : vec!()};
    }

    fn cycle(&mut self, instruction : Instruction) -> i32 {
        match instruction {
            Instruction::Noop => {},
            Instruction::Addx(_) => {
                self.executing_instructions.push((2, instruction));
            }
        }

        let register_value = self.value;

        let mut index = 0;
        while index < self.executing_instructions.len() {
            match &self.executing_instructions[index] {
                (0, ins) => {
                    match ins {
                        Instruction::Addx(val) => {
                            self.value += val;
                        },
                        _ => {},
                    };
                    self.executing_instructions.remove(index);
                },
                (num, ins) => {
                    self.executing_instructions[index].0 -= 1;
                    index += 1;
                }
            }

        }

        return register_value;
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

fn execute_instructions(register : &mut Register, instructions : Vec<Instruction>) {
    for (index, instr) in instructions.into_iter().enumerate() {
        let val = register.cycle(instr);
        if (index+20)%40 == 0 {
            println!("{} {}", index, val);
        }
    }
}


fn main() {
    println!("Hello, world!");
    let instructions = file_to_instuctions(String::from("files/instructions_test.txt"));
    // println!("{:?}", instructions);

    let mut register = Register::new(1);
    execute_instructions(&mut register, instructions);
    println!("{}", register.value);
}
