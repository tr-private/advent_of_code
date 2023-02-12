use std::{
    path::Path,
    fs,
    str,
};

fn main() {
    let file_path = Path::new("input.txt");
    if !file_path.is_file(){
        panic!("input.txt does not exist");
    }

    let file_content = fs::read_to_string(file_path).expect(&format!("cannot read file input.txt"));

    println!("input.txt has {} chars", file_content.len());
    let mut max_elve_calories = 0;
    let mut current_elve_calories = 0;
    let mut elve_count = 0;

    for line in (file_content + "\n").lines() {
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            if current_elve_calories > max_elve_calories {
                max_elve_calories = current_elve_calories
            }
            current_elve_calories = 0;
            elve_count += 1;
        }
        else {
            current_elve_calories += str::parse::<i32>(trimmed_line).expect( &format!("cannot parse line:{}", line));
        }

    }

    println!("{} elves found", elve_count);
    println!("{} calories carried by the elve carrying the most", max_elve_calories);


}
