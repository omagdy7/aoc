use std::{collections::HashMap, str::FromStr, string::ParseError};

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
}

#[derive(Debug, Clone)]
enum FileType {
    File(File),
    Dir(Directory),
}

impl FromStr for File {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<_> = s.split(" ").collect();
        let (size, name) = (
            splitted[0].parse::<usize>().unwrap(),
            splitted[1].to_string(),
        );
        Ok(File { name, size })
    }
}

impl FromStr for Directory {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<_> = s.split(" ").collect();
        let (_, name) = (splitted[0], splitted[1].to_string());
        Ok(Directory { name })
    }
}

impl FromStr for FileType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_file(s) {
            Ok(FileType::File(File::from_str(s).unwrap()))
        } else {
            Ok(FileType::Dir(Directory::from_str(s).unwrap()))
        }
    }
}

enum CommandType {
    ChangeDir,
    ListDir,
}

struct Command {
    command_type: CommandType,
    dir: Directory,
}

fn is_command(s: &str) -> bool {
    s.contains("$ cd")
}

fn is_file(s: &str) -> bool {
    s.split(" ").nth(0).unwrap().parse::<usize>().is_ok()
}

fn is_dir(s: &str) -> bool {
    s.contains("dir")
}

fn calculate_size_part_one(
    dir: String,
    dirs: &HashMap<String, Vec<FileType>>,
    list_of_dirs_under_100k: &mut Vec<usize>,
) -> usize {
    let mut size: usize = 0;
    let files = dirs.get(&dir).unwrap();
    for file in files {
        match file {
            FileType::File(cur_file) => {
                size += cur_file.size;
            }
            FileType::Dir(cur_dir) => {
                let dir_sz =
                    calculate_size_part_one(cur_dir.name.clone(), dirs, list_of_dirs_under_100k);
                if dir_sz <= 100000 {
                    list_of_dirs_under_100k.push(dir_sz)
                }
                size += dir_sz;
            }
        }
    }
    size
}

fn calculate_size_part_two(
    dir: String,
    dirs: &HashMap<String, Vec<FileType>>,
    list_of_dirs: &mut Vec<usize>,
) -> usize {
    let mut size: usize = 0;
    let files = dirs.get(&dir).unwrap();
    for file in files {
        match file {
            FileType::File(cur_file) => {
                size += cur_file.size;
            }
            FileType::Dir(cur_dir) => {
                let dir_sz = calculate_size_part_two(cur_dir.name.clone(), dirs, list_of_dirs);
                list_of_dirs.push(dir_sz);
                size += dir_sz;
            }
        }
    }
    size
}

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<_> = s.split(" ").collect();
        let (_, cmd, dir) = (splitted[0], splitted[1], splitted[2]);
        Ok(Command {
            command_type: match cmd {
                "cd" => CommandType::ChangeDir,
                "ls" => CommandType::ListDir,
                _ => unreachable!(),
            },
            dir: Directory {
                name: dir.to_string(),
            },
        })
    }
}

fn solve_part_one(data: &str) -> usize {
    let mut dirs: HashMap<String, Vec<FileType>> = HashMap::new();
    let mut cwd: String = "/".to_string();
    for line in data.lines() {
        if is_command(line) {
            let cmd = Command::from_str(line).unwrap();
            match cmd.command_type {
                CommandType::ChangeDir => {
                    if cmd.dir.name == ".." && cwd != "/" {
                        let mut pos_of_last_backslash = cwd.len();
                        for i in (0..cwd.len() - 2).rev() {
                            if cwd.get(i..=i).unwrap() == "/" {
                                pos_of_last_backslash = i + 1;
                                break;
                            }
                        }
                        cwd = cwd[0..pos_of_last_backslash].to_string()
                    } else if cmd.dir.name != "/" {
                        cwd += format!("{}/", cmd.dir.name).as_str();
                    }
                }
                CommandType::ListDir => {}
            }
        }
        if is_file(line) {
            dirs.entry(cwd.clone())
                .or_insert(vec![])
                .push(FileType::File(File::from_str(line).unwrap()))
        } else if is_dir(line) {
            let dir_name = Directory::from_str(line).unwrap().name;
            let cur_path = cwd.clone() + format!("{}/", dir_name).as_str();
            dirs.entry(cwd.clone())
                .or_insert(vec![])
                .push(FileType::Dir(Directory { name: cur_path }))
        }
    }
    let mut v: Vec<_> = vec![];
    let _ = calculate_size_part_one("/".to_string(), &dirs, &mut v);
    v.iter().sum()
}

fn solve_part_two(data: &str) -> usize {
    let mut dirs: HashMap<String, Vec<FileType>> = HashMap::new();
    let mut cwd: String = "/".to_string();
    for line in data.lines() {
        if is_command(line) {
            let cmd = Command::from_str(line).unwrap();
            match cmd.command_type {
                CommandType::ChangeDir => {
                    if cmd.dir.name == ".." && cwd != "/" {
                        let mut pos_of_last_backslash = cwd.len();
                        for i in (0..cwd.len() - 2).rev() {
                            if cwd.get(i..=i).unwrap() == "/" {
                                pos_of_last_backslash = i + 1;
                                break;
                            }
                        }
                        cwd = cwd[0..pos_of_last_backslash].to_string()
                    } else if cmd.dir.name != "/" {
                        cwd += format!("{}/", cmd.dir.name).as_str();
                    }
                }
                CommandType::ListDir => {}
            }
        }
        if is_file(line) {
            dirs.entry(cwd.clone())
                .or_insert(vec![])
                .push(FileType::File(File::from_str(line).unwrap()))
        } else if is_dir(line) {
            let dir_name = Directory::from_str(line).unwrap().name;
            let cur_path = cwd.clone() + format!("{}/", dir_name).as_str();
            dirs.entry(cwd.clone())
                .or_insert(vec![])
                .push(FileType::Dir(Directory { name: cur_path }))
        }
    }
    let mut v: Vec<_> = vec![];
    let total_space = 70_000_000;
    let wanted_unused_space = 30_000_000;
    let used_space = calculate_size_part_two("/".to_string(), &dirs, &mut v);
    let unused_space = total_space - used_space;
    *v.iter()
        .filter(|&x| x + unused_space >= wanted_unused_space)
        .min()
        .unwrap()
}

fn main() {
    let data_test = include_str!("../input/day7.test");
    let data_prod = include_str!("../input/day7.prod");
    println!("{}", solve_part_one(data_test));
    println!("{}", solve_part_one(data_prod));
    println!("{}", solve_part_two(data_test));
    println!("{}", solve_part_two(data_prod));
}
