use super::file_util;
use std::rc::Rc;
use std::cell::RefCell;

//TODO: make so get_or_add_dir will actually add instead of just returning curr

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

    fn add_dir_to_path(&mut self, child : Dir, path: Vec<String>) {
        // self.dirs.push(child);
        // if path.len() == 0 {
        //     self.add_dir(child);
        //     return;
        // }
        let mut curr = self;
        for next_path in path {
            curr = curr.get_or_add_dir(next_path);
        }
        curr.add_dir(child);
    }
    fn get_or_add_dir(&mut self, name : String) -> &mut Dir {
        for i in 0..self.dirs.len() {
            if self.dirs[i].name == name {
                return &mut self.dirs[i];
            }
        }
        // let mut new_dir = Dir::new(name);
        // let ret = &mut new_dir;
        // self.dirs.push(new_dir);
        // return ret;
        //
        


        // let new_dir = Box::new(Dir::new(name)); // Store the new directory on the heap
        //
        // // Get a mutable reference to the directory in the Box
        // let new_dir_ref: &mut Dir = Box::as_mut(new_dir);
        //
        // self.dirs.push(*new_dir_ref); // Add the directory to the list
        // new_dir_ref // Return a mutable reference to the new directory/


        return self;

        // for dir in self.dirs.iter() {
        //     if dir.name == name {
        //         return Some(dir);
        //     }
        // }
        // return None;
    }

    fn add_file(&mut self, child : File) {
        self.files.push(child);
    }

    fn add_file_to_path(&mut self, child : File, path : Vec<String>) {
        let mut curr = self;
        for next_path in path {
            curr = curr.get_or_add_dir(next_path);
        }
        curr.add_file(child);
    }

    fn get_dir(&self, name : String) -> Option<&Dir> {
        for dir in self.dirs.iter() {
            if dir.name == name {
                return Some(dir);
            }
        }
        return None;
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
    // CD(String),
    // LS((Vec<String>, Vec<(i32, String)>)),
    CD { directory_name : String},
    LS { directory_names : Vec<String>, file_infos : Vec<(i32, String)> },
}

fn lines_to_commands (lines : Vec<String>) -> Vec<Command>{
    let mut commands = vec!();
    let words = file_util::split_on_char(lines[0].clone(), ' ');
    match words[1].as_str() {
        "cd" => {
            let command = Command::CD{directory_name : words[2].clone()};
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
            let command = Command::LS{directory_names : dirs, file_infos : files};
            commands.push(command);
            commands.extend(remaining_commands);
        },
        _ => {},
    };
    return commands;
}


fn create_directory_structure (commands : Vec<Command>) -> Dir {
    let mut root = Dir::new(String::from("/"));
    // let curr : &mut Dir = &mut root;
    let mut cwd_path : Vec<String> = vec!();
    for command in commands {
        match command {
            Command::CD {directory_name : name} => {
                match name.as_str() {
                    "/" => {cwd_path = vec!();},
                    ".." => {cwd_path.pop();},
                    _ => {
                        cwd_path.push(name);
                    },
                }
                // let next_dir = match curr.get_dir(name.clone()) {
                //     None => {
                //         // let new_dir = Dir::new(name);
                //         // root.add_dir(new_dir);
                //         // curr = new_dir;
                //     },
                //     Some(got_dir) => {
                //
                //         // curr = got_dir;
                //     },
                // };
            },
            Command::LS {directory_names, file_infos} => {
                for dir_name in directory_names {
                    root.add_dir_to_path(Dir::new(dir_name), cwd_path.clone());
                }
                for (file_size, file_name) in file_infos {
                    root.add_file_to_path(File::new(file_name, file_size ), cwd_path.clone());
                }
            }
            _ => {

            }
        }
    }
    // println!("cwd_path: {:?}", cwd_path);

    // let mut dir1 =  dir::new(string::from("dir1"));
    // let dir1sub =  dir::new(string::from("dir1sub"));
    // dir1.add_dir(dir1sub);
    // root.add_dir(dir1);
    // let dir2 =  box::new(dir::new(string::from("dir2")));
    // root.add_dir(dir2);
    // let rootfile1 = file::new(string::from("rootfile1"), 10);
    // root.add_file(rootfile1);
    root
}

fn total_at_most_100k_sum (root : &Dir) -> (i32, i32) {
    let mut size = 0;
    let mut sum = 0;
    for dir in root.dirs.iter() {
        let (dir_sum, dir_size) = total_at_most_100k_sum(dir);
        size += dir_size;
        sum += dir_sum;
    }
    for file in root.files.iter() {
        sum += file.size;
    }
    if(size<= 100000){
        return (sum + size, size);
    }
    return (0, size);


}

pub fn part1 (lines : Vec<String>) {
    let commands = lines_to_commands(lines);
    // for command in commands {
    //     println!("{:?}", command);
    // }
    //todo 
    let directory_stucture = create_directory_structure(commands);
    // directory_stucture.print();

    let (result, _) = total_at_most_100k_sum(&directory_stucture);
    println!("{}", result);
}
