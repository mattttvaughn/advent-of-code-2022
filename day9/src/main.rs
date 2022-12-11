use std::collections::HashSet;

type Point = (i32, i32);

fn main() {
    let input = include_str!("input.txt");

    // pt. 1
    calc_rope_tail_locations(input, vec![(0, 0), (0, 0)]);

    // pt. 2
    calc_rope_tail_locations(input, (0..10).map(|_| (0, 0)).collect());
}

fn calc_rope_tail_locations(input: &str, mut rope: Vec<(i32, i32)>) {
    let mut tail_location_set: HashSet<Point> = HashSet::new();
    tail_location_set.insert((0, 0));

    for line in input.lines() {
        let (dir, magnitude) = line.split_once(" ").unwrap();
        let magnitude: usize = magnitude.parse().unwrap();
        for _ in 0..magnitude {
            // move head
            let head_move = match dir {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => panic!("fail"),
            };

            let head = rope[0];
            rope[0] = (head.0 + head_move.0, head.1 + head_move.1);
            adjust_rope(&mut rope);
            tail_location_set.insert(*rope.last().unwrap());
        }
    }
    println!("{}", tail_location_set.len());
}

// Adjusts child knots in a rope to account a moved head knot
fn adjust_rope(knots: &mut Vec<Point>) {
    // skip parent, function assumes it has already been moved
    for i in 1..knots.len() {
        let parent_knot = knots[i - 1];
        let child_knot = knots[i];
        knots[i] = move_knot(parent_knot, child_knot);
    }
}

fn move_knot(parent_loc: Point, child_loc: Point) -> Point {
    let parent_child_diff = (parent_loc.0 - child_loc.0, parent_loc.1 - child_loc.1);
    if (parent_child_diff.0).abs() < 2 && (parent_child_diff.1).abs() < 2 {
        // adjacent, child doesn't need to move
        return child_loc;
    }
    // resolve tail if not already adjacent
    let child_move = calc_move(&parent_child_diff);
    return (child_loc.0 + child_move.0, child_loc.1 + child_move.1);
}

// Compute where how a child knot should be moved given the offset from its parent.
fn calc_move((x_offset, y_offset): &(i32, i32)) -> (i32, i32) {
    return (
        x_offset.signum() * x_offset.abs().min(1),
        y_offset.signum() * y_offset.abs().min(1),
    );
}
