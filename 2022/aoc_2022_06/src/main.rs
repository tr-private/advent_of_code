use std::{
    path::Path,
    fs,
    collections::LinkedList,
};

fn main() {
    let file_path = Path::new("input.txt");
    if !file_path.is_file(){
        panic!("input.txt does not exist");
    }

    let file_content: Vec<char> =
        fs::read_to_string(file_path)
            .expect(&format!("cannot read file input.txt"))
            .chars()
            .collect();

    let mut last_chars = LinkedList::from( ['!' ; 14]);

    let mut cursor_position = 0;
    for current_char in file_content{
        cursor_position += 1;
        last_chars.push_front(current_char);
        last_chars.pop_back();

        if cursor_position > last_chars.len() && !has_duplicate(&last_chars){
            println!("first position with 4 different chars is {}", cursor_position);
            return;
        }
    }

}

fn has_duplicate(list : &LinkedList<char>) -> bool{
    for check_char in list{
        let mut check_char_count = 0;
        for char in list
        {
            if check_char == char {
                check_char_count += 1;
            }
        }
        if check_char_count > 1{
            return true;
        }
    }

    false
}