fn main() {
    let input = include_str!("input.txt");

    let size = 4;
    let mut dupe_index : i32 = -1;
    for i in (size - 1)..input.len() {
        // check the past [size] characters
        let mut window: Vec<char> = input[(i - (size - 1))..=i].chars().collect::<Vec<char>>();
        println!("{}", window.iter().collect::<String>());
        window.sort();
        let mut has_dupe = false;
        for window_idx in 1..window.len() {
            if window[window_idx - 1] == window[window_idx] {
                dupe_index = (i as i32) + 2;
                has_dupe = true;
            }
        }
        if !has_dupe {
            break;
        }
    }

    println!("{}", dupe_index)


}
