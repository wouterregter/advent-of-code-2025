fn in_bounds(grid: &[Vec<char>], i: i32, j: i32) -> bool {
    i >= 0 && i < grid.len() as i32 && j >= 0 && j < grid[0].len() as i32
}

fn remove_rolls(mut grid: Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
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
    let mut pos_to_remove = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '@' {
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
                pos_to_remove.push((i, j));
                // println!("accessible roll at pos {i}, {j}");
            }
        }
    }

    for (i, j) in pos_to_remove {
        grid[i][j] = '.';
    }
    return (grid, accessible_roll_count);
}

fn main() -> std::io::Result<()> {
    let contents = std::fs::read_to_string("data/day4/input.txt")?;

    // part 1

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (_, accessible_roll_count) = remove_rolls(grid);
    println!("answer part 1: {accessible_roll_count}");

    // part 2

    let mut grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut removed_rolls = 0;

    loop {
        let (new_grid, accessible_roll_count) = remove_rolls(grid);
        if accessible_roll_count < 1 {
            break;
        }
        removed_rolls += accessible_roll_count;
        grid = new_grid;
    }

    println!("answer part 2: {removed_rolls}");

    // println!("{:?}", grid);
    Ok(())
}
