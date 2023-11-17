

struct Monkey {
    number : i32,
    items : Vec<i32>,
    operation : Box<dyn Fn(i32) -> i32>,
    test : Box<dyn Fn(i32) -> i32>,
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

fn main() {
    println!("Hello, world!");
    let  monkeys = file_to_monkeys(String::from("files/monkey_test.txt"));
    for (i,m) in monkeys.into_iter().enumerate() {
        println!("{:?}", m.number);
        println!("{:?}", m.items);
        // println!("{}, {}", i, (m.operation)(5));
        // println!("{}, {}", i, (m.operation)(10));
        // println!("{}, {}", i, (m.test)(13));
    }
}
