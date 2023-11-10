// need to have it be executing cycles

#[derive(Debug)]
#[derive(Clone)]
enum Instruction{
    Noop,
    Addx(i32),
}

struct Register {
    value : i32,
    instructions : Vec<Instruction>,
    executing_instructions : Vec<(i32, Instruction)>,
}

impl Register {
    fn new(starting_value : i32, instructions : Vec<Instruction>) -> Register {
        return Register{value : starting_value, instructions, executing_instructions : vec!()};
    }

    fn execute_all(&mut self) -> i32 {
        let mut cycle_num = 0;
        let mut total_signal_strength = 0;

        while self.instructions.len() + self.executing_instructions.len() > 0 {
            cycle_num += 1;
            let value_during_cycle = self.cycle();
            if (cycle_num -20)%40 == 0 {
                let signal_strength = cycle_num * value_during_cycle;
                total_signal_strength += signal_strength;
                // println!("{} {}", cycle_num, value_during_cycle);
           }
        }
        // println!("{} {}", cycle_num, self.value);
        return total_signal_strength;

    }

    fn add_execute_instr(&mut self, instr : Instruction) {
        match instr {
            Instruction::Noop => {
                self.executing_instructions.push((0,instr));
            }
            Instruction::Addx(_) => {
                self.executing_instructions.push((1,instr));
            }
        }
    }

    fn cycle(&mut self) -> i32 {


        if self.executing_instructions.len() == 0 {
            match self.instructions.first() {
                None => {},
                Some(instr) => {
                    self.add_execute_instr(instr.clone());
                    self.instructions.remove(0);
                },
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
                (_num, _ins) => {
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

// fn execute_instructions(register : &mut Register, instructions : Vec<Instruction>) {
//     for (index, instr) in instructions.into_iter().enumerate() {
//         // let val = register.cycle(instr);
//         if (index+20)%40 == 0 {
//             println!("{} {}", index, val);
//         }
//     }
// }


fn main() {
    let instructions = file_to_instuctions(String::from("files/instructions.txt"));
    // let instructions = file_to_instuctions(String::from("files/instructions_test.txt"));
    // let instructions = file_to_instuctions(String::from("files/short.txt"));
    // println!("{:?}", instructions);

    let mut register = Register::new(1, instructions);
    let sum = register.execute_all();
    println!("{}", sum);
    // execute_instructions(&mut register, instructions);
    // println!("{}", register.value);
}
