use std::{
    path::Path,
    fs,
};

fn main() {
    let file_path = Path::new("input.txt");
    if !file_path.is_file(){
        panic!("input.txt does not exist");
    }

    let file_content = fs::read_to_string(file_path).expect(&format!("cannot read file input.txt"));

    let mut stacks = Vec::new();

    stacks.push(get_stack(&file_content,'1'));
    stacks.push(get_stack(&file_content,'2'));
    stacks.push(get_stack(&file_content,'3'));
    stacks.push(get_stack(&file_content,'4'));
    stacks.push(get_stack(&file_content,'5'));
    stacks.push(get_stack(&file_content,'6'));
    stacks.push(get_stack(&file_content,'7'));
    stacks.push(get_stack(&file_content,'8'));
    stacks.push(get_stack(&file_content,'9'));

    let mut stacks1 = stacks.clone();

    for line in (file_content.clone() + "\n").lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("move") {
            stacks1 = Move::from_line(trimmed_line).execute_with_crane_mover_9000(stacks1);
        }
    }

    let mut stacks2 = stacks.clone();
    for line in (file_content.clone() + "\n").lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("move") {
            stacks2 = Move::from_line(trimmed_line).execute_with_crane_mover_9001(stacks2);
        }
    }

    let mut result1 = "".to_string();
    for stack in stacks1{
        result1 = format!("{}{}",
                         result1,
                         stack.clone().pop().expect("Stack does not contain a crate"));
    }
    println!("Top of crates using CrateMover 9000: {}", result1);

    let mut result2 = "".to_string();
    for stack in stacks2{
        result2 = format!("{}{}",
                          result2,
                          stack.clone().pop().expect("Stack does not contain a crate"));
    }
    println!("Top of crates using CrateMover 9001: {}", result2);

}

#[derive(Debug)]
struct Position{
    line:i32,
    column:i32,
}

#[derive(Debug)]
struct Move{
    from:usize,
    to: usize,
    crates: usize,
}

impl Move{
    fn from_line (line: &str) -> Self{
        let split: Vec<&str> = line.split(' ').collect();
        if split.len() != 6{
            panic!("invalid line (split_count) \"{}\"", line)
        }
        let crates = str::parse::<usize>(split[1]).expect( &format!("cannot parse crates:{}", line));
        let from = str::parse::<usize>(split[3]).expect( &format!("cannot from crates:{}", line)) - 1 ;
        let to = str::parse::<usize>(split[5]).expect( &format!("cannot to crates:{}", line)) - 1;

        Move{crates, from, to}
    }
    fn execute_with_crane_mover_9000(&self, stacks: Vec<Vec<char>>) -> Vec<Vec<char>>
    {
        let mut result = Vec::new();
        let mut from_stack = stacks[self.from].clone();
        let mut to_stack = stacks[self.to].clone();

        let mut crates = self.crates;

        //println!("{:?}", self);
        //println!("from {:?}", &from_stack);
        //println!("to  {:?}", &to_stack);

        while crates > 0{
            let current_crate = from_stack.pop().expect("crate is empty, cannot move");
            to_stack.push(current_crate);
            crates -= 1;
        }
        //println!("from {:?}", &from_stack);
        //println!("from {:?}", &to_stack);
        //println!();

        let mut i = 0;
        for stack in stacks{
            if self.from == i{
                result.push(from_stack.clone());
            }
            else if  self.to == i{
                result.push(to_stack.clone());
            }
            else {
                result.push(stack);
            }
            i += 1;
        }

        result
    }

    fn execute_with_crane_mover_9001(&self, stacks: Vec<Vec<char>>) -> Vec<Vec<char>>
    {
        let mut result = Vec::new();
        let mut from_stack = stacks[self.from].clone();
        let mut to_stack = stacks[self.to].clone();

        let mut crates = self.crates;

        //println!("{:?}", self);
        //println!("from {:?}", &from_stack);
        //println!("to  {:?}", &to_stack);

        let mut cargo_stack = Vec::new();
        while crates > 0{

            let current_crate = from_stack.pop().expect("crate is empty, cannot move");
            cargo_stack.push(current_crate);
            crates -= 1;
        }

        while let Some(current_crate) = cargo_stack.pop() {
            to_stack.push(current_crate);
        }
        //println!("from {:?}", &from_stack);
        //println!("from {:?}", &to_stack);
        //println!();

        let mut i = 0;
        for stack in stacks{
            if self.from == i{
                result.push(from_stack.clone());
            }
            else if  self.to == i{
                result.push(to_stack.clone());
            }
            else {
                result.push(stack);
            }
            i += 1;
        }

        result
    }
}


fn get_stack(text: &String, label: char) -> Vec<char> {
    let start_position = get_first_position(text, label).expect("stack with given label not found");
    let mut stack = Vec::new();
    let mut current_position = Position{
        line: start_position.line - 1,
        column: start_position.column
    } ;

    while current_position.line >= 0 {
        let current_char = get_char_at(text, &current_position);
        if current_char != ' ' {
            stack.push(current_char);
        }

        current_position = Position{
            line: current_position.line - 1,
            column: current_position.column
        } ;
    }
    stack
}
fn get_char_at(text:&String, position: &Position) -> char {
    let mut line_index = 0;

    for line in text.lines() {
        if line_index == position.line {
            let mut column_index = 0;
            for current_char in line.chars(){
                if column_index == position.column{
                    return  current_char;
                }
                column_index += 1;
            }
        }
        line_index += 1;
    }

    panic!("Position {:?} is out of range", position);
}

fn get_first_position( text:&String, search: char ) -> Option<Position>{
    let mut line_index = 0;

    for line in text.lines() {
        let mut column_index = 0;
        for current_char in line.chars(){
            if current_char == search {
                return Some(Position{
                    line: line_index,
                    column: column_index
                });
            }
            column_index += 1;
        }

        line_index += 1;
    }

    None
}