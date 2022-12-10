use std::iter::{Zip, repeat};

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("{:?}", input);

    // Strategy:
    //  1) iterate over all elements
    //  2) check all rows/columns towards edges for visibility
    //  3) count the above checks, btw

    let mut visible_count = 0;
    let mut max_scenic_score = 0;
    let width = input.len();
    let height = input[0].len();
    for x in 0..width {
        for y in 0..height {
            // iterate over row/column
            let curr_height = input[x][y];
            let is_blocked_to_left = (0..x).any(|x1| input[x1][y] >= curr_height) && x != 0;
            let is_blocked_to_right = ((x + 1)..width).any(|x1| input[x1][y] >= curr_height) && x != (width - 1);
            let is_blocked_above = (0..y).any(|y1| input[x][y1] >= curr_height) && y != 0;
            let is_blocked_below = ((y + 1)..height).any(|y1| input[x][y1] >= curr_height) && y != (height - 1);
            if !is_blocked_to_left || !is_blocked_to_right || !is_blocked_above || !is_blocked_below {
                visible_count += 1;
            } else if x == 0 || x == width || y == 0 || y == height {
                visible_count += 1;
            } 

            let scenic_score_left = calc_scenic_score(&input, (0..x).rev().zip(repeat(y)).collect::<Vec<(usize, usize)>>(), curr_height);
            let scenic_score_right = calc_scenic_score(&input, ((x+1)..width).zip(repeat(y)).collect::<Vec<(usize, usize)>>(), curr_height);
            let scenic_score_top = calc_scenic_score(&input, (repeat(x)).zip((0..y).rev()).collect::<Vec<(usize, usize)>>(), curr_height);
            let scenic_score_bottom= calc_scenic_score(&input, (repeat(x)).zip((y+1)..height).collect::<Vec<(usize, usize)>>(), curr_height);
            let scenic_score = scenic_score_left * scenic_score_right * scenic_score_top * scenic_score_bottom;
            println!("({}, {}): {} * {} * {} * {} = {}", x, y, scenic_score_left, scenic_score_right, scenic_score_top, scenic_score_bottom, scenic_score);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }

        }
    }

    println!("Pt. 1: {}", visible_count);
    println!("Pt. 2: {}", max_scenic_score);

}

// [points] is ordered by order in which they should be compared
fn calc_scenic_score(map: &Vec<Vec<char>>, points: Vec<(usize, usize)>, curr_height: char) -> usize {
    for (i, (x, y)) in points.iter().enumerate() {
        let xy_height = map[*x][*y];
        if xy_height >= curr_height {
            return i + 1;
        }
    }
    return points.len();
}