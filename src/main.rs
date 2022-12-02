use std::fs;

fn main() {
    let data = fs::read_to_string("/home/matt/Development/advent2022/src/input01.txt")
        .expect("Unable to read file");

    let mut groups : Vec<i32> = Vec::new();
    let mut group_acc = 0;
    for line in data.lines() {
        if line.is_empty() {
            groups.push(group_acc);
            group_acc = 0;
        } else {
            let line_val : i32 = line.parse().expect("goodbye world");
            group_acc = group_acc + line_val;
        }
    }
    groups.sort();
    groups.reverse();

    println!("pt. 1: {}", groups[0]);
    println!("pt. 2: {}", groups[0] + groups[1] + groups[2])
}
