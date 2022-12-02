use std::{fs, collections::HashMap};

fn main() {
    let data = fs::read_to_string("/home/matt/Development/advent2022/day02/src/input.txt")
        .expect("Unable to read file");

    let mut pt1_map : HashMap<&str, i32> = HashMap::new();
    pt1_map.insert("A X", 4); //draw
    pt1_map.insert("A Y", 8); //win
    pt1_map.insert("A Z", 3); //loss
    pt1_map.insert("B X", 1); //loss
    pt1_map.insert("B Y", 5); //draw
    pt1_map.insert("B Z", 9); //win
    pt1_map.insert("C X", 7); //win
    pt1_map.insert("C Y", 2); //loss
    pt1_map.insert("C Z", 6); //draw

    let mut pt2_map : HashMap<&str, i32> = HashMap::new();
    pt2_map.insert("A X", 3); //rock + scissors = lose,
    pt2_map.insert("A Y", 4); //rock + rock = draw,
    pt2_map.insert("A Z", 8); //rock + paper = win,
    pt2_map.insert("B X", 1); //paper + rock = lose,
    pt2_map.insert("B Y", 5); //paper + paper = draw,
    pt2_map.insert("B Z", 9); //paper + scissors = win
    pt2_map.insert("C X", 2); //scissors + paper = lose
    pt2_map.insert("C Y", 6); //scissors + scissors = draw
    pt2_map.insert("C Z", 7); //scissors + rock = win


    let mut score_pt1 = 0;
    let mut score_pt2 = 0;
    for game in data.lines() {
        score_pt1 += pt1_map.get(game)
         .expect(format!("unknown matchup: {}", game).as_str());
        score_pt2 += pt2_map.get(game)
         .expect(format!("unknown matchup: {}", game).as_str());
    }

    println!("pt. 1: {}", score_pt1);
    println!("pt. 2: {}", score_pt2);
}
