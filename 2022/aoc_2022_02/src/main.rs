use std::{
    path::Path,
    fs,
    collections::btree_map::BTreeMap,
};

fn main() {
    let file_path = Path::new("input.txt");
    if !file_path.is_file(){
        panic!("input.txt does not exist");
    }

    let file_content = fs::read_to_string(file_path).expect(&format!("cannot read file input.txt"));

    let mut possible_results = BTreeMap::new();
    // rock vs rock
    possible_results.insert("A X", 1 + 3);
    // rock vs paper
    possible_results.insert("A Y", 2 + 6);
    // rock vs scissor
    possible_results.insert("A Z", 3 + 0);

    // paper vs rock
    possible_results.insert("B X", 1 + 0);
    // paper vs paper
    possible_results.insert("B Y", 2 + 3);
    // paper vs scissor
    possible_results.insert("B Z", 3 + 6);

    // scissor vs rock
    possible_results.insert("C X", 1 + 6);
    // scissor vs paper
    possible_results.insert("C Y", 2 + 0);
    // scissor vs scissor
    possible_results.insert("C Z", 3 + 3);

    println!("input.txt has {} chars", file_content.len());

    let mut score = 0;
    let mut matches_played = 0;
    for line in (file_content.clone() + "\n").lines() {
        let trimmed_line = line.trim();
        match possible_results.get(trimmed_line){
            Some(result) => {
                score += result;
                matches_played += 1;
            },
            None => {
                if trimmed_line != ""{
                    panic!("unknown situation: {}", trimmed_line);
                }
            },
        };
    }

    println!("{} matches played", matches_played);
    println!("My score is {} with given response", score);


    let mut possible_results = BTreeMap::new();
    // rock - lose  --> rock vs scissor
    possible_results.insert("A X", 3 + 0);
    // rock - draw  --> rock vs rock
    possible_results.insert("A Y", 1 + 3);
    // rock - win   --> rock vs paper
    possible_results.insert("A Z", 2 + 6);

    // paper - lose --> paper vs rock
    possible_results.insert("B X", 1 + 0);
    // paper - draw --> paper vs paper
    possible_results.insert("B Y", 2 + 3);
    // paper - win  --> paper vs scissor
    possible_results.insert("B Z", 3 + 6);

    // scissor - lose --> scissor vs paper
    possible_results.insert("C X", 2 + 0);
    // scissor - draw --> scissor vs scissor
    possible_results.insert("C Y", 3 + 3);
    // scissor - win  --> scissor vs rock
    possible_results.insert("C Z", 1 + 6);

    println!("input.txt has {} chars", file_content.len());

    let mut score = 0;
    let mut matches_played = 0;
    for line in (file_content.clone() + "\n").lines() {
        let trimmed_line = line.trim();
        match possible_results.get(trimmed_line){
            Some(result) => {
                score += result;
                matches_played += 1;
            },
            None => {
                if trimmed_line != ""{
                    panic!("unknown situation: {}", trimmed_line);
                }
            },
        };
    }

    println!("{} matches played", matches_played);
    println!("My score is {} with given outcome", score);

}
