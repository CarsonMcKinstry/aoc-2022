use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

const FILE_SYSTEM_MAX: i32 = 70000000;
const REQUIRED_SIZE: i32 = 30000000;

#[derive(PartialEq, Debug)]
pub struct File {
    pub name: String,
    pub t: FileType,
    pub size: Option<i32>,
    pub children: Vec<Rc<RefCell<File>>>,
    pub parent: Option<Rc<RefCell<File>>>,
}

#[derive(PartialEq, Debug)]
pub enum FileType {
    File,
    Dir,
}

impl File {
    pub fn new_file(name: &str, size: i32) -> Self {
        Self {
            name: name.to_string(),
            size: Some(size),
            t: FileType::File,
            children: vec![],
            parent: None,
        }
    }

    pub fn new_dir(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: None,
            t: FileType::Dir,
            children: vec![],
            parent: None,
        }
    }

    pub fn try_from_str(maybe_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut parts = maybe_file.split_whitespace();

        let file_size = parts.next().unwrap().parse::<i32>()?;
        let file_name = parts.next().unwrap();

        Ok(Self::new_file(file_name, file_size))
    }

    pub fn print(&self) -> String {
        match self.t {
            FileType::File => {
                format!("({} , {})", self.name, self.size.unwrap())
            }
            FileType::Dir => {
                format!(
                    "[{} , {}]",
                    self.name,
                    self.children
                        .iter()
                        .map(|tn| tn.borrow().print())
                        .collect::<Vec<String>>()
                        .join(" , ")
                )
            }
        }
    }

    pub fn final_size(&self) -> i32 {
        match self.t {
            FileType::File => self.size.unwrap(),
            FileType::Dir => self.children.iter().map(|c| c.borrow().final_size()).sum(),
        }
    }
}

struct FileSystem {
    root: Rc<RefCell<File>>,
}

impl FileSystem {
    pub fn recreate_crom_terminal(terminal: &str) -> Self {
        let root = Rc::new(RefCell::new(File::new_dir("/")));
        let mut current = Rc::clone(&root);

        for line in terminal.lines() {
            let line = line.trim();

            if line.starts_with("$ cd") {
                let parts = line.split_whitespace();
                let name = parts.last().unwrap();

                match name {
                    ".." => {
                        let current_clone = Rc::clone(&current);
                        current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                    }
                    name => {
                        let next_dir = Rc::new(RefCell::new(File::new_dir(name)));

                        current.borrow_mut().children.push(Rc::clone(&next_dir));

                        {
                            let mut mut_next_dir = next_dir.borrow_mut();
                            mut_next_dir.parent = Some(Rc::clone(&current));
                        }

                        current = next_dir
                    }
                }
            } else {
                let try_file = File::try_from_str(line);

                match try_file {
                    Ok(file) => {
                        let file = Rc::new(RefCell::new(file));
                        current.borrow_mut().children.push(Rc::clone(&file))
                    }
                    _ => {}
                }
            }
        }

        Self { root }
    }

    pub fn calculate_dir_sizes(&self) -> Vec<i32> {
        let file = Rc::clone(&self.root);
        let mut sizes: Vec<i32> = vec![];
        let mut stack: Vec<Rc<RefCell<File>>> = vec![];

        stack.push(file);

        while let Some(f) = stack.pop() {
            match f.borrow().t {
                FileType::Dir => {
                    let mut children = f.borrow().children.clone().into_iter().map(|child| child);

                    stack.extend(children);

                    sizes.push(f.borrow().final_size());
                }
                _ => {}
            }
        }

        sizes
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open input");

    let file_system = FileSystem::recreate_crom_terminal(input.as_str());

    let sizes = file_system.calculate_dir_sizes();

    let file_system_size = sizes.get(0).unwrap();

    let free_space = FILE_SYSTEM_MAX - file_system_size;

    let space_left_to_free = REQUIRED_SIZE - free_space;

    println!("File System Size:         {}", file_system_size);
    println!("Free Space:               {}", free_space);
    println!("Space Left To Free:       {}", space_left_to_free);

    let part_one: i32 = sizes.iter().filter(|size| size <= &&100_000).sum();

    let part_two = sizes
        .iter()
        .filter(|size| size >= &&space_left_to_free)
        .min()
        .unwrap();

    println!("{part_one}");
    println!("{part_two}");
}
