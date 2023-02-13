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
    let mut pair_count = 0;
    let mut overlapping_pair_count = 0;
    let mut full_overlapping_pair_count = 0;
    for line in (file_content + "\n").lines() {
        let trimmed_line = line.trim();
        if trimmed_line != "" {
            pair_count += 1;

            let pair = Pair::from_str(line);
            if pair.has_overlap() {
                overlapping_pair_count += 1;
                //println!(" + {}", trimmed_line)
            }
            else{
                //println!(" - {}", trimmed_line)
            }
            if pair.is_full_overlap() {
                full_overlapping_pair_count += 1;
                //println!(" + {}", trimmed_line)
            }
            else{
                //println!(" - {}", trimmed_line)
            }

        }
    }

    println!("{} pairs found", pair_count);
    println!("{} pairs with overlap found", overlapping_pair_count);
    println!("{} pairs with complete overlap found", full_overlapping_pair_count);



}
struct Range{
    from:i32,
    to:i32,
}
impl Range{
    fn from_str(input: &str) -> Self{
        let split_input = input.split("-").collect::<Vec<&str>>();
        if split_input.len() != 2{
            panic!("Cannot parse Range from \"{}\". split_input.len() != 2", input);
        }
        let from = str::parse(split_input[0]).expect( &format!("Cannot parse range from \"{}\". Cannot parse split_input[0].", input));
        let to = str::parse(split_input[1]).expect( &format!("Cannot parse range from \"{}\". Cannot parse split_input[1].", input));
        Range{ from, to}
    }
}

struct Pair{
    range1: Range,
    range2: Range,
}
impl Pair{
    fn from_str(input: &str) -> Self{
        let split_input = input.split(",").collect::<Vec<&str>>();
        if split_input.len() != 2{
            panic!("Cannot parse Pair from \"{}\". split_input.len() != 2", input);
        }
        let range1 = Range::from_str(split_input[0]);
        let range2 = Range::from_str(split_input[1]);
        Pair{ range1, range2}
    }

    fn is_full_overlap(&self) -> bool{
        if self.range1.from <= self.range2.from && self.range1.to >= self.range2.to{
            // range1 encloses range2
            true
        }
        else if self.range2.from <= self.range1.from && self.range2.to >= self.range1.to{
            // range2 encloses range1
            true
        }
        else{
            false
        }
    }

    fn has_overlap(&self) -> bool{
        if self.range1.from <= self.range2.from && self.range1.to >= self.range2.from{
            true
        }
        else if self.range2.from <= self.range1.from && self.range2.to >= self.range1.from{
            true
        }
        else{
            false
        }
    }
}

