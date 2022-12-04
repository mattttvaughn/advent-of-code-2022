use std::{fs, collections::HashSet, hash::Hash};


fn main() {
    let data = fs::read_to_string("/home/matt/Development/advent2022/day04/src/input.txt")
        .expect("Unable to read file");

    let mut fully_overlapping_pair_count = 0;
    let mut partially_overlapping_pair_count = 0;
    for line in data.lines() {
        let line_split : Vec<&str> = line.split(",").collect();
        let (range_str_1, range_str_2) = (line_split[0], line_split[1]);
        let (range_1_set, range_2_set) = (str_to_range_elem_set(range_str_1), str_to_range_elem_set(range_str_2));
        if does_either_set_fully_contain_other(&range_1_set, &range_2_set) {
            fully_overlapping_pair_count += 1;
        }
        if range_1_set.intersection(&range_2_set).count() != 0 {
            partially_overlapping_pair_count += 1;
        }
    }

    println!("Pt. 1: {}", fully_overlapping_pair_count);
    println!("Pt. 2: {}", partially_overlapping_pair_count);

}

fn str_to_range_elem_set(s: &str) -> HashSet<i32> {
    let items : Vec<&str> = s.split("-").collect();
    let (range_start_str, range_end_str) = (items[0], items[1]);
    let range = range_start_str.parse().unwrap()..=range_end_str.parse().unwrap();
    return range.collect()
}

fn does_either_set_fully_contain_other<T: Hash + Eq>(set1: &HashSet<T>, set2: &HashSet<T>) -> bool {
    let union_size = set1.union(&set2).count();
    return union_size == set1.len() || union_size == set2.len();
}