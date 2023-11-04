use super::file_util;

struct File {
    name : String,
    size : i32,
}

impl File {
    fn new(name: String, size : i32) -> File {
        File {
            name : name,
            size : size,
        }
    }
    fn print(&self) {
        println!("{} {}", self.name, self. size);
    }
    fn print_full(&self, spacing : String) {
        println!("{}{} {}", spacing, self.name, self. size);
    }
}

struct Dir {
    name : String,
    // parent : Option<Box<Dir>>,
    // parent : Option<&Dir>,
    dirs : Vec<Dir>,
    files : Vec<File>,
}

impl Dir {
    fn new(name : String) -> Dir {
        Dir {
            name : name,
            // parent : None,
            dirs : vec!(),
            files : vec!(),
        }
    }


    fn add_dir(&mut self, child : Dir) {
        // child.parent = Some(self);
        self.dirs.push(child);
    }

    fn add_file(&mut self, child : File) {
        self.files.push(child);
    }

    fn print(&self) {
        self.print_full(String::new());
    }

    fn print_full(&self, spacing : String) {
        println!("{}{} (dir)", spacing, self.name);
        let mut new_spacing = spacing;
        new_spacing.push_str("   ");
        for dir in self.dirs.iter() {
            dir.print_full(new_spacing.clone());
        }
        for file in self.files.iter() {
            file.print_full(new_spacing.clone());
        }

    }
}

#[derive(Debug)] 
enum Command {
    CD(String),
    LS((Vec<String>, Vec<(i32, String)>)),
}

fn lines_to_commands (lines : Vec<String>) -> Vec<Command>{
    let mut commands = vec!();
    let words = file_util::split_on_char(lines[0].clone(), ' ');
    match words[1].as_str() {
        "cd" => {
            let command = Command::CD(words[2].clone());
            commands.push(command);
            let remaining_lines = lines[1..lines.len()].to_vec();
            commands.extend(lines_to_commands(remaining_lines));
        },
        "ls" => {
            let mut dirs = vec!();
            let mut files = vec!();
            // commands.push(command);
            let mut remaining_commands : Vec<Command>= vec!();
            for i in 1..lines.len() {
                let line_words = file_util::split_on_char(lines[i].clone(), ' ');
                match line_words[0].as_str() {
                    "$" => {
                        let remaining_lines = lines[i..lines.len()].to_vec();
                        remaining_commands = lines_to_commands(remaining_lines);
                        // commands.extend(lines_to_commands(remaining_lines));
                        break;
                    },
                    "dir" => {
                        dirs.push(line_words[1].clone());
                    },
                    _ => {
                        files.push((line_words[0].parse::<i32>().unwrap(), line_words[1].clone()));
                    },
                }
            }
            let command = Command::LS((dirs, files));
            commands.push(command);
            commands.extend(remaining_commands);
        },
        _ => {},
    };
    return commands;
}

pub fn part1 (lines : Vec<String>) {
    let commands = lines_to_commands(lines);
    for command in commands {
        println!("{:?}", command);
    }
    //TODO 
    // let directory_stucture = create_directory_structure(commands);
    // directory_stucture.print(); 
}

fn create_directory_structure (commands : Vec<Command>) -> Dir {
    let mut root = Dir::new(String::from("/"));

    // let mut dir1 =  Dir::new(String::from("dir1"));
    // let dir1sub =  Dir::new(String::from("dir1sub"));
    // dir1.add_dir(dir1sub);
    // root.add_dir(dir1);
    // let dir2 =  Dir::new(String::from("dir2"));
    // root.add_dir(dir2);
    // let rootfile1 = File::new(String::from("rootfile1"), 10);
    // root.add_file(rootfile1);
    root
}
