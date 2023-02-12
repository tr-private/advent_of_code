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

    println!("input.txt has {} chars", file_content.len());
    let mut rucksack_count = 0;
    let mut priority_sum = 0;

    for line in (file_content.clone() + "\n").lines() {
        let trimmed_line = line.trim();
        if trimmed_line != "" {

            let item_list : Vec<char> = trimmed_line.chars().collect();
            let rucksack_size = trimmed_line.len();
            let compartment_size = rucksack_size / 2;
            let mut matching_char = '!';
            for position_in_compartment_1 in 0..=compartment_size - 1 {
                for position_in_compartment_2 in compartment_size..=rucksack_size - 1 {
                    // match found
                    // match not found before
                    if item_list[position_in_compartment_1] == item_list[position_in_compartment_2] &&
                        matching_char != item_list[position_in_compartment_1] {

                        if matching_char != '!' {
                            panic!("more than one match found {}, previous_match {}, current_match {}", trimmed_line, matching_char, item_list[position_in_compartment_1]);
                        }

                        matching_char = item_list[position_in_compartment_1];
                        let priority;
                        if (matching_char as u16) > 96 && (matching_char as u16) <= 97 +26  {
                            priority = (matching_char as u16) - 96;
                        }
                        else if (matching_char as u16) > 64 && (matching_char as u16) <= 97 +26  {
                            priority = (matching_char as u16) - 64 + 26;
                        }
                        else{
                            panic!("unexpected character {}", matching_char)
                        }

                        // println!("line {}, match {}, priority {}", trimmed_line, matching_char, priority);
                        priority_sum += priority;
                    }
                }
            }
            rucksack_count += 1;
        }
    }

    println!("{} rucksacks analyzed", rucksack_count);
    println!("priority_sum is {}", priority_sum);

    let mut priority_sum = 0;

    let mut groups: Vec<(String, String, String)> = vec!();
    let mut current_group = ("".to_string(), "".to_string(), "".to_string());
    for line in (file_content.clone() + "\n").lines() {
        let trimmed_line = line.trim();
        if trimmed_line != "" {
            if rucksack_count % 3 == 0 {
                current_group.0 = trimmed_line.to_string();
            } else if rucksack_count % 3 == 1 {
                current_group.1 = trimmed_line.to_string();
            } else if rucksack_count % 3 == 2 {
                current_group.2 = trimmed_line.to_string();
                groups.push(current_group);
                current_group = ("".to_string(), "".to_string(), "".to_string());
            }
            rucksack_count += 1;
        }
    }

    for group in &groups{
        if group.0 == "" || group.1 == "" || group.2 == "" {
            panic!("One of the rucksacks is empty \"{}\" \"{}\" \"{}\"", group.0, group.1, group.2)
        }

        let mut matching_char = '!';
            //println!("{} {} {}", group.0, group.1, group.2);
            for item_in_group0 in group.0.chars() {
                for item_in_group1 in group.1.chars() {
                    for item_in_group2 in group.2.chars() {
                        //println!("- {} {} {}", item_in_group0, item_in_group0, item_in_group2);

                        // match found
                        // match not found before
                        if item_in_group0 == item_in_group1 &&
                            item_in_group0 == item_in_group2 &&
                            matching_char != item_in_group0 {
                            if matching_char != '!' {
                                panic!("more than one match found: previous_match {}, current_match {}", matching_char, item_in_group0);
                            }

                            matching_char = item_in_group0;
                            let priority;
                            if (matching_char as u16) > 96 && (matching_char as u16) <= 97 + 26 {
                                priority = (matching_char as u16) - 96;
                            } else if (matching_char as u16) > 64 && (matching_char as u16) <= 97 + 26 {
                                priority = (matching_char as u16) - 64 + 26;
                            } else {
                                panic!("unexpected character {}", matching_char)
                            }

                            priority_sum += priority;
                        }
                    }
                }
            }
        }

    println!("{} groups analyzed", groups.len());
    println!("priority_sum of team batches is {}", priority_sum);

}
