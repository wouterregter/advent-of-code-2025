use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("data/day2/input.txt")?;

    // part 1

    let mut answer = 0;

    let ranges = contents.split(",");
    for range in ranges {
        // println!("range: {range}");
        let numbers: Vec<_> = range.split("-").collect();
        let start = numbers[0].parse::<i64>().unwrap();
        let end = numbers[1].parse::<i64>().unwrap();

        for number in start..end + 1 {
            // println!("checking number {number} for repeated numbers");
            let number_str = number.to_string();
            let number_length = number_str.len();

            let split_pos = number_length / 2;
            let first_part = &number_str[0..split_pos];
            let second_part = &number_str[split_pos..];

            if first_part == second_part {
                // println!("first part {first_part} equal to second part {second_part}");
                answer += number;
                // println!("added number {number} to answer");
            } else {
                // println!("first part {first_part} not equal to second part {second_part}");
            }
        }
    }

    println!("answer part 1: {answer}");

    // part 2

    let mut answer = 0;

    let ranges = contents.split(",");
    for range in ranges {
        // println!("range: {range}");
        let numbers: Vec<_> = range.split("-").collect();
        let start = numbers[0].parse::<i64>().unwrap();
        let end = numbers[1].parse::<i64>().unwrap();

        for number in start..end + 1 {
            let number_str = number.to_string();
            let number_length = number_str.len();

            for pattern_length in 1..(number_length / 2) + 1 {
                let pattern = &number_str[0..pattern_length];
                // println!("checking number: {number_str} with for pattern: {pattern}");

                if number_length % pattern_length != 0 {
                    // println!(
                    //     "number_length {number_length} not divisible by pattern length {pattern_length}"
                    // );
                    continue;
                }

                let mut add_number = true;
                for pos in
                    (pattern_length..number_length - pattern_length + 1).step_by(pattern_length)
                {
                    let part = &number_str[pos..pos + pattern_length];

                    if part != pattern {
                        // println!("part {part} no match with pattern {pattern}");
                        add_number = false;
                        break;
                    }
                    // println!("part {part} matches pattern")
                }
                if add_number {
                    answer += number;
                    // println!("all parts passed added {number_str} to answer");
                    break;
                }
            }
        }
    }

    println!("answer part 2: {answer}");

    Ok(())
}
