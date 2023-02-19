use std::{
    path::Path,
    fs,
};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use crate::ParserMode::{ParsingCommand, ParsingLsOutput};

fn main() {
    let file_path = Path::new("input.txt");
    if !file_path.is_file(){
        panic!("input.txt does not exist");
    }

    let file_content = fs::read_to_string(file_path).expect(&format!("cannot read file input.txt"));

    let disk_space = 70000000;
    let update_size = 30000000;

    let mut all_directories = Vec::new();
    let root = Directory::new_root();
    all_directories.push(root.clone());
    let mut working_directory = root.clone();
    let mut mode = ParsingCommand;


    for line in file_content.lines(){
        if line.starts_with("$"){
            mode = ParsingCommand;
        }

        if mode == ParsingLsOutput {
            let line_split: Vec<&str> = line.split(' ').collect();
            if line_split.len() != 2 {
                panic!("unexpected ls line {}", line)
            }

            if line_split[0] =="dir"{
                println!("Parsing dir line {}", line);
                if let Some(new_directory) = Directory::add_directory(&working_directory, &line_split[1].to_string()){
                    all_directories.push(new_directory);
                }
            }
            else {
                println!("Parsing file line {}", line);
                Directory::add_file(&working_directory, &line_split[1].to_string(),str::parse::<i32>(line_split[0]).expect("cannot parse file size") );
            }

        }

        else if mode == ParsingCommand && line == "$ cd /" {
            println!("\"$ cd /\" command: {}", line);
            working_directory = root.clone();
        }
        else if line == "$ cd .." {
            println!("\"$ cd .. \" command: {}", line);
            let copy = working_directory.clone();
            let mut locked_copy = copy.lock();
            let parent_option = &mut locked_copy.parent_directory;
            let parent = parent_option.take().unwrap();
            working_directory = parent;

            println!("working_directory is {}", locked_copy.name);
        }
        else if line.starts_with("$ cd") {
            println!("\"$ cd <name_parameter>\" command: {}", line);
            let line_split: Vec<&str> = line.split(' ').collect();
            if line_split.len() != 3 {
                panic!("unexpected \"$ cd\" command {}", line)
            }
            let name_parameter = line_split[2];
            let copy = working_directory.clone();
            let mut locked_copy = copy.lock();
            let child_option = &mut locked_copy.sub_directories.get_mut(name_parameter);
            let child = child_option.take().expect("subdirectory not found");
            working_directory = child.clone();

            println!("working_directory is {}", locked_copy.name);
        }
        else if line.starts_with("$ ls") {
            println!("\"$ ls\" command: {}", line);
            mode = ParsingLsOutput;
        }
        else {
            panic!("unexpected command: {}", line);
        }

    }

    let root_size = Directory::get_size(&root);
    println!("root hast total size of {}", root_size);
    let mut sum_of_dirs = 0;
    for dir in &all_directories{
        if Directory::get_size(&dir) <= 100000 {
            sum_of_dirs += Directory::get_size(&dir);
        }
    }
    println!("sum of small directories total size is {}", sum_of_dirs);

    let unused_before_deletion = disk_space - root_size;
    println!("unused disk space is");
    println!("{}", unused_before_deletion);

    let delete_requirement = update_size - unused_before_deletion;
    println!("we need to delete");
    println!("{}", delete_requirement);

    let mut dir_size_to_delete = root_size;
    for dir in &all_directories{
        let current_dir_size = Directory::get_size(&dir);
        if current_dir_size >= delete_requirement && current_dir_size < dir_size_to_delete{
            dir_size_to_delete = current_dir_size;
        }
    }
    println!("we will delete a directory with total size of");
    println!("{}", dir_size_to_delete);
    println!("unused disk space after deletion will be");
    println!("{}", root_size - dir_size_to_delete);

}

struct Directory{
    parent_directory:Option<SuperReference<Directory>>,
    name: String,

    total_size:Option<i32>,

    sub_directories:BTreeMap<String, SuperReference<Directory>>,
    files:BTreeMap<String,File>,
}


impl Directory{
    fn new(parent: &SuperReference<Directory>, name: &String) -> SuperReference<Self> {
        SuperReference::new( Directory{
            parent_directory: Some(parent.clone()),
            name: name.to_string(),

            sub_directories: BTreeMap::new(),
            files: BTreeMap::new(),

            total_size: None,

        })
    }

    fn new_root() -> SuperReference<Self> {
        SuperReference::new( Directory{
            parent_directory: None,
            name: String::new(),

            sub_directories: BTreeMap::new(),
            files: BTreeMap::new(),

            total_size: None,
        })
    }

    fn add_file(me: &SuperReference<Self>, name: &String, file_size: i32){
        if me.lock().files.contains_key(name){ return; }
        me.lock().files.insert(
            name.clone(),
            File{name: name.clone(), size: file_size});
    }
    fn add_directory(me: &SuperReference<Self>, name: &String) -> Option<SuperReference<Directory>>{
        if me.lock().sub_directories.contains_key(name){
            return None
        }
        else {
            let new_directory = Directory::new(me, name);
            me.lock().sub_directories.insert(
                name.clone(),
                new_directory.clone()
            );

            return Some(new_directory);
        }
    }

    fn get_size(me: &SuperReference<Self>) -> i32{
        if let Some(size) = me.lock().total_size{
            return size;
        }
        let mut size = 0;
        for dir in me.lock().sub_directories.values(){
            size += Directory::get_size(&dir);
        }
        for file in me.lock().files.values(){
            size += file.size;
        }
        me.lock().total_size = Some(size);
        return size;
    }
}

struct File{
    name: String,
    size: i32,
}

struct SuperReference<T>{
    obj: Arc<Mutex<T>>,
}

impl <T> SuperReference<T>{
    fn new(obj: T) -> Self{
        SuperReference{
            obj: Arc::new(Mutex::new(obj))
        }
    }

    fn clone(&self) -> Self{
        SuperReference{
            obj: self.obj.clone()
        }
    }
    fn lock(&self) -> MutexGuard<T>{
        self.obj.lock().unwrap()
    }

}

#[derive(PartialEq)]
enum ParserMode{
    ParsingCommand,
    ParsingLsOutput,
}