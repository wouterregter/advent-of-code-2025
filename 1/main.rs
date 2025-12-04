use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Part 1

    let mut counter = 0;
    let mut current = 50;
    for line in contents.lines() {
        let direction = &line[0..1];
        let magnitude = &line[1..].parse::<i32>().unwrap();
        
        match direction {
            "L" => current = (current - magnitude + 100).rem_euclid(100),
            "R" => current = (current + magnitude).rem_euclid(100),
            _ => {}
        }

        if current == 0 {
            counter += 1
        }
    
    };
    println!("answer part 1: {counter}");

    // Part 2

    let mut counter = 0;
    let mut current = 50;
    for line in contents.lines() {
        let direction = &line[0..1];
        let magnitude = &line[1..].parse::<i32>().unwrap();
        
        match direction {
            "L" => {
                if (current - magnitude) <= 0 {
                    let mut added_to_counter = 0;
                    if current != 0 {
                        added_to_counter += 1
                    }
                    added_to_counter += ((current - magnitude) / 100).abs();
                    // println!("current: {current}, dir: {direction}, mag: {magnitude} added to counter: {added_to_counter}");
                    counter += added_to_counter;
                } else {
                    // println!("current: {current}, dir: {direction}, mag: {magnitude} added to counter: 0");
                }
                current = ((current - magnitude).rem_euclid(100))
                ;
            },
            "R" => {
                if (current + magnitude) > 99 {
                    let mut added_to_counter = ((current + magnitude) / 100);
                    // println!("current: {current}, dir: {direction}, mag: {magnitude} added to counter: {added_to_counter}");
                    counter += added_to_counter;
                }
                current = (current + magnitude).rem_euclid(100);
            }
            _ => {}
        }
    
    };
    println!("answer 2: {counter}");

    Ok(())
}