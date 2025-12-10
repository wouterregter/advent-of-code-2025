use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let contents = std::fs::read_to_string("data/day5/input.txt")?;

    let (ranges, ids) = contents.split_once("\n\n").expect("Invalid input format");

    let mut ranges: Vec<(i64, i64)> = ranges
        .split("\n")
        .filter_map(|line| {
            let (start, end) = line.split_once("-")?;
            Some((start.parse().ok()?, end.parse().ok()?))
        })
        .collect();
    let ids: Vec<i64> = ids.split("\n").filter_map(|s| s.parse().ok()).collect();
    println!("ranges: {ranges:?}\nids: {ids:?}");

    // part 1

    let mut fresh_counter = 0;
    'outer: for id in &ids {
        for (start, end) in &ranges {
            if id >= start && id <= end {
                println!("id: {id} is fresh");
                fresh_counter += 1;
                continue 'outer;
            }
        }
    }
    println!("answer: {fresh_counter}");

    // part 2

    ranges.sort();
    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    let mut prev_start = &ranges[0].0;
    let mut prev_end = &ranges[0].1;
    let mut merged_range_before = false;
    let mut merged_range_start = 0;
    let mut merged_range_end = 0;

    for (cur_start, cur_end) in &ranges[1..] {
        println!("cur: {cur_start}-{cur_end}, prev: {prev_start}-{prev_end}");
        if cur_start <= &(prev_end + 1) {
            if merged_range_before {
                merged_range_end = merged_range_end.max(*cur_end);
            } else {
                merged_range_start = *prev_start;
                merged_range_end = (*prev_end).max(*cur_end);

                merged_range_before = true;
            }
        } else {
            if merged_range_before {
                println!("added {merged_range_start}, {merged_range_end}");
                merged_ranges.push((merged_range_start, merged_range_end));
            } else {
                println!("added {prev_start}, {prev_end}");
                merged_ranges.push((*prev_start, *prev_end));
            }
            merged_range_before = false;
        }

        prev_start = cur_start;
        prev_end = cur_end;
    }

    if merged_range_before {
        println!("added {merged_range_start}, {merged_range_end}");
        merged_ranges.push((merged_range_start, merged_range_end));
    } else {
        println!("added {prev_start}, {prev_end}");
        merged_ranges.push((*prev_start, *prev_end));
    }

    println!("ranges {:?}", merged_ranges);

    let total_count: i64 = merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    println!("answer part 2: {}", total_count);

    Ok(())
}
