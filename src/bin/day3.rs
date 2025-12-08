use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("data/day3/input.txt")?;

    // let mut answer = 0;
    // for line in contents.lines() {
    //     println!("{line}");
    //     let mut numbers: Vec<i64> = Vec::new();
    //     for number in line.chars() {
    //         numbers.push(number.to_string().parse::<i64>().unwrap());
    //     }
    //     let first_max_value = numbers.iter().max().unwrap();
    //     let first_max_position: usize =
    //         numbers.iter().position(|&n| n == *first_max_value).unwrap();

    //     let remaining_numbers = if first_max_position == numbers.len() - 1 {
    //         &numbers[..first_max_position]
    //     } else {
    //         &numbers[first_max_position + 1..]
    //     };

    //     println!("{:?}", first_max_value);
    //     println!("{:?}", remaining_numbers);

    //     let second_max_value = remaining_numbers.iter().max().unwrap();

    //     let value_to_add = if first_max_position == numbers.len() - 1 {
    //         second_max_value * 10 + first_max_value
    //     } else {
    //         first_max_value * 10 + second_max_value
    //     };
    //     println!("added {value_to_add}");
    //     answer += value_to_add;
    // }

    let mut answer = 0;
    for line in contents.lines() {
        println!("{line}");
        let mut numbers: Vec<i64> = Vec::new();
        for number in line.chars() {
            numbers.push(number.to_string().parse::<i64>().unwrap());
        }

        let mut on_indices: Vec<usize> = Vec::new();

        for i in 0..12 {
            let (max_position, max_value) = numbers
                .iter()
                .enumerate()
                .max_by_key(|(_, &value)| value)
                .map(|(pos, &val)| (pos, val))
                .unwrap();

            on_indices.push(max_position);

            let numbers = if max_position == numbers.len() - 1 {
                &numbers[..max_position]
            } else {
                &numbers[max_position + 1..]
            };

            numbers.remove(max_position);
        }

        println!("{:?}", max_position);
        println!("{:?}", remaining_numbers);

        println!("added {value_to_add}");
        answer += value_to_add;
    }

    println!("answer: {answer}");
    Ok(())
}
