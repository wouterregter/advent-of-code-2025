fn in_bounds(grid: &[Vec<char>], i: i32, j: i32) -> bool {
    i >= 0 && i < grid.len() as i32 && j >= 0 && j < grid[0].len() as i32
}

fn main() -> std::io::Result<()> {
    let contents = std::fs::read_to_string("data/day4/input.txt")?;

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let dirs: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut accessible_roll_count = 0;

    for (i, _) in grid.iter().enumerate() {
        for (j, _) in grid[0].iter().enumerate() {
            if grid[i as usize][j as usize] != '@' {
                continue;
            }
            let mut roll_count = 0;

            for (di, dj) in &dirs {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if in_bounds(&grid, ni, nj) && grid[ni as usize][nj as usize] == '@' {
                    roll_count += 1
                }
            }
            if roll_count < 4 {
                accessible_roll_count += 1;
                println!("accessible roll at pos {i}, {j}");
            }
        }
    }

    println!("{accessible_roll_count}");

    // println!("{:?}", grid);
    Ok(())
}
