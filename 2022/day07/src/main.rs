use std::fs::File as FsFile;
use std::io::{self, BufRead};
use std::iter::Peekable;
use std::string::String;

struct File {
    name: String,
    bytes: usize,
}

struct Directory {
    name: String,
    subdirs: Vec<Self>,
    files: Vec<File>,
    local_size: usize,
    total_size: usize,
}

fn read_directory_files<T: Iterator<Item=io::Result<String>>>(lines: &mut Peekable<T>, files: &mut Vec<File>) -> usize {
    let mut local_size: usize = 0;
    loop {
        let line = lines.peek();
        // Could be the end of the file, handle this correctly
        if let None = line {
            return local_size;
        }
        let line = line.unwrap().as_ref().unwrap();
        if line.starts_with('$') {
            return local_size;
        }
        let line: String = lines.next().unwrap().unwrap();
        let (k, v) = line.split_once(' ').unwrap();
        if k.starts_with('d') {
            continue;
        }
        let size: usize = usize::from_str_radix(k, 10).unwrap();
        local_size += size;
        files.push(File{name: String::from(v), bytes: size});
    }
}

fn read_directory_input() -> Directory {
    let file = io::BufReader::new(FsFile::open("input.txt").unwrap());
    let mut lines = file.lines().peekable();
    let mut count_lt: usize = 0;

    let mut root = Directory{
        name: String::new(),
        subdirs: Vec::new(),
        files: Vec::new(),
        local_size: 0,
        total_size: 0,
    };
    let line: String = lines.next().unwrap().unwrap();
    let token: Vec<&str> = line.split_whitespace().collect();
    assert_eq!(token[0], "$");
    assert_eq!(token[1], "cd");
    assert_eq!(token[2], "/");
    lines.next().unwrap().unwrap();
    root.name += "/";
    root.local_size = read_directory_files(&mut lines, &mut root.files);
    root.total_size = root.local_size;

    let mut dir_stack: Vec<Directory> = Vec::new();
    dir_stack.push(root);

    loop {
        let maybe_line = lines.next();
        if let None = maybe_line {
            break;
        }
        let line: String = maybe_line.unwrap().unwrap();
        let token: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(token.len(), 3);
        assert_eq!(token[0], "$");
        assert_eq!(token[1], "cd");
        if token[2] == ".." {
            let cwd = dir_stack.pop().unwrap();
            let mut parent = dir_stack.last_mut().unwrap();
            if cwd.total_size <= 100000 {
                count_lt += cwd.total_size;
            }
            parent.total_size += cwd.total_size;
            parent.subdirs.push(cwd);
            continue;
        }
        let mut this_dir = Directory{
            name: String::from(token[2]),
            subdirs: Vec::new(),
            files: Vec::new(),
            local_size: 0,
            total_size: 0,
        };
        lines.next().unwrap().unwrap(); // skip ls
        this_dir.local_size = read_directory_files(&mut lines, &mut this_dir.files);
        this_dir.total_size = this_dir.local_size;
        dir_stack.push(this_dir);
    }
    while dir_stack.len() > 1 {
        let cwd = dir_stack.pop().unwrap();
        let mut parent = dir_stack.last_mut().unwrap();
        if cwd.total_size <= 100000 {
            count_lt += cwd.total_size;
        }
        parent.total_size += cwd.total_size;
        parent.subdirs.push(cwd);
    }
    println!("Total size of dirs w/ total_size <= 100000: {}", count_lt);
    dir_stack.pop().unwrap()
}

fn find_dir_to_delete(root: &Directory) {
    const TOTAL_FS_SIZE: usize = 70000000;
    const NEEDED_SIZE: usize = 30000000;
    let amount_to_free: usize = NEEDED_SIZE - (TOTAL_FS_SIZE - root.total_size);
    let mut smallest_dir_size: usize = TOTAL_FS_SIZE;
    let mut stack: Vec<&Directory> = Vec::new();
    stack.push(&root);
    println!("Filesystem capacity is {}", TOTAL_FS_SIZE);
    println!("Filesystem size is     {}", root.total_size);
    println!("Need at least          {}", NEEDED_SIZE);
    println!("Free capacity is       {}", TOTAL_FS_SIZE - root.total_size);
    println!("Must free              {}", amount_to_free);
    while !stack.is_empty() {
        let dir = stack.pop().unwrap();
        println!("{} {} - {} {}", "  ".repeat(stack.len()), dir.name, dir.total_size, dir.local_size);
        if dir.total_size >= amount_to_free && dir.total_size < smallest_dir_size {
            println!("{} ->Dir would free {}", "  ".repeat(stack.len()), dir.total_size);
            smallest_dir_size = dir.total_size;
        }
        for child in dir.subdirs.iter() {
            stack.push(child);
        }
    }
    println!("Directory with smallest size to free up space: {}", smallest_dir_size);
}

fn main() {
    let root = read_directory_input();
    find_dir_to_delete(&root);
}
