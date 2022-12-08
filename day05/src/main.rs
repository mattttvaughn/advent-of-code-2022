fn main() {
    let data = include_str!("input.txt");

    let mut stacks = parse_stacks(data);

    let instruction_strings : Vec<&str> = data.lines()
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .collect();

    move_crates_individually(&instruction_strings, &mut stacks);

    let mut stacks2 = parse_stacks(data);
    move_crates_by_group(&instruction_strings, &mut stacks2);

    println!("pt. 1: {:?}", stacks.iter().map(|stack| stack.last().unwrap_or(&' ')).collect::<String>());
    println!("pt. 2: {:?}", stacks2.iter().map(|stack| stack.last().unwrap_or(&' ')).collect::<String>());

}

fn move_crates_by_group(instruction_strings: &Vec<&str>, stacks: &mut Vec<Vec<char>>) {
    for instruction_string in instruction_strings {

        let instructions = instruction_string.replace("move ", "");
        let instructions = instructions.split(" ")
            .filter(|&x| x != "from" && x != "to")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let (num_to_move, from, to) = (instructions[0], instructions[1], instructions[2]);

        let pop_stack = &mut stacks[from as usize - 1];
        let len = pop_stack.len() as usize;
        let range_start = len - num_to_move as usize;
        let mut popped = pop_stack[range_start..len].to_vec();

        pop_stack.truncate(pop_stack.len() - popped.len());
        stacks[to as usize - 1].append(&mut popped);
    }

}

fn move_crates_individually(instruction_strings: &Vec<&str>, stacks: &mut Vec<Vec<char>>) {
    for instruction_string in instruction_strings {

        let instructions = instruction_string.replace("move ", "");
        let instructions = instructions.split(" ")
            .filter(|&x| x != "from" && x != "to")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let (num_to_move, from, to) = (instructions[0], instructions[1], instructions[2]);
        // moves [num_to_move] crates from stack [from] to stack [to]. note that [from]
        // and [to] are 1-indexed
        for _ in 0..num_to_move {
            let popped = stacks[from as usize - 1].pop();
            match popped {
                Some(x) => stacks[to as usize - 1].push(x),
                None => {}
            }
        }
    }
}

fn parse_stacks(data: &str) ->  Vec<Vec<char>> {
    let stack_strings : Vec<&str> = data.lines()
        .take_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .filter(|line| line.replace(" ", "").parse::<i32>().is_err())
        .collect();

    let stack_indices : &str = data.lines()
        .take_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .last()
        .unwrap();

    let mut stacks : Vec<Vec<char>> = vec![];
    stack_indices.split_whitespace().for_each(|_i| { stacks.push(vec![]) });
    for (x_index, c) in stack_indices.chars().enumerate() {
        if !c.is_whitespace() {
            let stack_index = c.to_digit(10).unwrap() - 1;
            for stack_str in stack_strings.iter() {
                let char_at = stack_str.chars().nth(x_index).filter(|p| p.is_alphabetic());
                match char_at {
                    Some(good_char) => { stacks[stack_index as usize].insert(0, good_char as char) }
                    None => {}
                }
            }
        }
    }

    println!("stacks: {:?}", stacks);

    stacks
}


