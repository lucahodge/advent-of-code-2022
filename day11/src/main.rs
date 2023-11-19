

struct Monkey {
    number : i32,
    items : Vec<i32>,
    operation : Box<dyn Fn(i32) -> i32>,
    test : Box<dyn Fn(i32) -> i32>,
}

impl Monkey {
    fn inspect_and_throw (&mut self) -> Option<(i32, i32)> {
        if self.items.len() > 0 {
            let mut item = self.items[0];
            self.items.remove(0);
            //inspect
            item = (self.operation)(item);
            //relief
            item /= 3;
            //throw
            let monkey_number = (self.test)(item);
            return Some((monkey_number, item));
        }
        return None;
    }
    // fn turn(&mut self, monkeys : &mut Vec<Monkey>) {
    //
    //     while self.items.len() > 0 {
    //         let item = self.items[0];
    //         self.items.remove(0);
    //
    //
    //         //inspect
    //         //relief
    //         //throw
    //     }
    //
    // }
}

fn file_to_monkeys (file_name:String) -> Vec<Monkey> {
    let mut monkeys = vec!();
    let lines : Vec<String> = std::fs::read_to_string(file_name).unwrap().lines().map(String::from).collect();
    for i in (0..lines.len()).step_by(7) {

        //number
        // let number = lines[i].split(" ").last().unwrap().chars().take(1).collect::<String>().parse::<i32>().unwrap();
        // TODO: why do I have to collect before reversing again?
        let number = lines[i].split(" ").last().unwrap().chars().rev().skip(1).collect::<String>().chars().rev().collect::<String>().parse::<i32>().unwrap();
        //items:
        let mut items : Vec<i32> = vec!();
        for e in lines[i+1].split(": ").nth(1).unwrap().split(", ") {
            items.push(e.parse::<i32>().unwrap());
        }
        //operation:
        let symbol = lines[i+2].split(" ").nth(6).unwrap();
        let val = lines[i+2].split(" ").nth(7).unwrap();
        let operation = match (symbol, val) {
            ("*", "old") => {
                Box::new(move |x| x * x) as Box<dyn Fn(i32) -> i32>
            },
            ("*", _) => {
                let num = val.parse::<i32>().unwrap();
                Box::new(move |x| x * num) as Box<dyn Fn(i32) -> i32>
            },
            ("+", "old") => {
                Box::new(move |x| x + x) as Box<dyn Fn(i32)->i32>
            },
            ("+", _) => {
                let num = val.parse::<i32>().unwrap();
                Box::new(move |x| x + num) as Box<dyn Fn(i32)->i32>
            },
            _ => {
                Box::new(move |x| x) as Box<dyn Fn(i32)->i32>
            },
        };
        //test:
        let divisible_by = lines[i+3].split(" ").last().unwrap().parse::<i32>().unwrap();
        let if_true = lines[i+4].split(" ").last().unwrap().parse::<i32>().unwrap();
        let if_false = lines[i+5].split(" ").last().unwrap().parse::<i32>().unwrap();
        let test = Box::new(move |x| {
            if x % divisible_by == 0 {
                if_true
            } 
            else {
                if_false
            }
        }) as Box<dyn Fn(i32) -> i32>;

        let monkey = Monkey {
            number : number,
            items : items,
            operation : operation,
            test : test,
        };
        monkeys.push(monkey);
    }
    return monkeys;
}

fn round(monkeys : &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        while monkeys[i].items.len() > 0 {
            match monkeys[i].inspect_and_throw() {
                Some((monkey_number, item)) => {
                    for i in 0..monkeys.len() {
                        if monkey_number == monkeys[i].number {
                            monkeys[i].items.push(item);
                        }
                    }
                },
                None => {},
            }
        }
    }
}

fn number_of_inspections_per_moneky (monkeys : &mut Vec<Monkey>, num_rounds : i32) -> Vec<i32> {
    let mut inspections : Vec<i32> = vec![0; monkeys.len()];
    for _ in 0..num_rounds{
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i].items.len() as i32;
            while monkeys[i].items.len() > 0 {
                match monkeys[i].inspect_and_throw() {
                    Some((monkey_number, item)) => {
                        for i in 0..monkeys.len() {
                            if monkey_number == monkeys[i].number {
                                monkeys[i].items.push(item);
                            }
                        }
                    },
                    None => {},
                }
            }
        }
    }
    return inspections;
}

fn monkey_business(inspections: Vec<i32>) -> i32 {
    let mut max1 : (usize, i32) = (0,0);
    for (i,e) in inspections.iter().enumerate() {
        if *e > max1.1 {
            max1 = (i,*e)
        }
    }
    let mut max2 = (0,0);
    for (i,e) in inspections.iter().enumerate() {
        if *e > max2.1 && i != max1.0 {
            max2 = (i,*e)
        }
    }
    return max1.1 * max2.1;
}

fn main() {
    let mut  monkeys = file_to_monkeys(String::from("files/monkey.txt"));
    let inspections = number_of_inspections_per_moneky(&mut monkeys, 20);
    // for (i,m) in monkeys.iter().enumerate() {
    //     println!("{:?} {:?}", m.number, m.items);
    // }
    let monkey_business = monkey_business(inspections);
    println!("{}", monkey_business);
    // for inspec in inpections {
    //     println!("{:?}", inspec);
    // }

    // round(&mut monkeys);
    // for (i,m) in monkeys.iter().enumerate() {
    //     println!("{:?} {:?}", m.number, m.items);
    // }
}
