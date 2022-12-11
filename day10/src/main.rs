fn main() {
    let lines = include_str!("input.txt").lines();

    let mut reg_x: i32 = 1;
    let mut cycle_count = 0;
    let mut signal_strength_count = 0;

    println!("Pt.2: ");
    for line in lines {
        let parsed = line.split_once(" ");

        let (op, amount) = match parsed {
            Some((_op, _amt)) => (_op, _amt.parse().expect("Unparseable!")),
            None => (line, -1 as i32),
        };

        match op {
            "addx" => {
                increment_cycle(2, &mut cycle_count, &reg_x, &mut signal_strength_count);
                reg_x += amount;
            }
            "noop" => {
                increment_cycle(1, &mut cycle_count, &reg_x, &mut signal_strength_count);
            }
            _ => panic!("Unknown operation in {}", line),
        }
    }
    println!();
    println!("Pt.1: {}", signal_strength_count);
}

fn increment_cycle(
    cycles_to_increment: i32,
    cycle_count: &mut i32,
    reg_x: &i32,
    signal_strength_acc: &mut i32,
) {
    for _ in 0..cycles_to_increment {
        *cycle_count += 1;
        if *cycle_count == 20 || (*cycle_count - 20) % 40 == 0 {
            *signal_strength_acc = *signal_strength_acc + (reg_x * (*cycle_count));
        }
        // does sprite intersect current crt pixel?
        let current_crt_pixel = *cycle_count % 40;
        let sprite_start = *reg_x;
        let sprite_end = *reg_x + 2;
        if current_crt_pixel >= sprite_start && current_crt_pixel <= sprite_end {
            print!("#");
        } else {
            print!(".");
        }
        if current_crt_pixel == 0 {
            println!()
        }
    }
}
