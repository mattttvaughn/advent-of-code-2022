use std::{fs, collections::HashSet};

fn main() {
    let data = fs::read_to_string("/home/matt/Development/advent2022/day03/src/input.txt")
        .expect("Unable to read file");

    let mut pt1_sum = 0;
    let mut first_char_set : HashSet<char> = HashSet::new();
    let mut second_char_set : HashSet<char> = HashSet::new();
    for line in data.lines() {

        let (first_half, second_half) = line.split_at(line.len() / 2);
        str_to_set(first_half, &mut first_char_set);
        str_to_set(second_half, &mut second_char_set);
        let shared_char = first_char_set.intersection(&second_char_set).last().unwrap();
        pt1_sum += find_char_priority_value(&shared_char);
    }
    
    println!("Pt. 1: {}",  pt1_sum);

    let group_size = 3;
    let lines : Vec<&str> = data.lines().collect();
    let mut group_start_index = 0;
    let mut pt2_sum = 0;
    let mut third_char_set : HashSet<char> = HashSet::new();
    let mut intersection_char_set : HashSet<char> = HashSet::new();
    loop {
        let first = lines.get(group_start_index).unwrap();
        let second = lines.get(group_start_index + 1).unwrap();
        let third = lines.get(group_start_index + 2).unwrap();
        str_to_set(first, &mut first_char_set);
        str_to_set(second, &mut second_char_set);
        str_to_set(third, &mut third_char_set);
        let intersection1 = first_char_set.intersection(&second_char_set);
        str_to_set(intersection1.collect::<String>().as_str(), &mut intersection_char_set);
        let intersection2 = intersection_char_set.intersection(&third_char_set);
        let intersection_2_str = intersection2.collect::<String>();

        pt2_sum += find_char_priority_value(&intersection_2_str.chars().last().unwrap());

        group_start_index += group_size;
        if group_start_index >= lines.len() { // this would be wrong for groups not divisible by [group_size]
            break;
        }
    }

    println!("Pt. 2: {}", pt2_sum)
}

fn str_to_set(str: &str, set: &mut HashSet<char>) {
    set.clear();
    for c in str.chars() {
        set.insert(c);
    }
}

// calculate value for priority character: a-z -> 1-26; A-Z -> 27-52
fn find_char_priority_value(shared_char: &char) -> u32 {
    return match shared_char {
        'a'..='z' => {
            (*shared_char as u32) - ('a' as u32) + 1 // add 1 since 'a' == 1
        }
        'A'..='Z' => {
            (*shared_char as u32) - ('A' as u32) + 27 // add 27 since 'A' == 27
        }
        _ => panic!("Could not find priority value for character: {}", shared_char)
    };
}
