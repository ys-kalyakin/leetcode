#[allow(dead_code)]
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let mut fresh_oranges = 0;

    let mut coords: Vec<(i32, i32)> = Vec::new();

    for (i, v) in grid.iter().enumerate() {
        for (j, val) in v.iter().enumerate() {
            match val {
                1 => fresh_oranges += 1,
                2 => coords.push((i as i32, j as i32)),
                _ => continue,
            }
        }
    }

    let dir = vec![vec![-1i32, 0i32], vec![1, 0], vec![0, -1], vec![0, 1]];
    let mut time_to_infect = 0;

    let mut grid = grid;
    while coords.len() != 0 {
        for _ in 0..coords.len() {
            let (x, y) = coords.remove(0);

            for d in &dir {
                let next_x = d[0] + x;
                let next_y = d[1] + y;

                if next_x >= 0
                    && next_x < grid.len() as i32
                    && next_y >= 0
                    && next_y < grid[0].len() as i32
                {
                    if grid[next_x as usize][next_y as usize] == 1 {
                        coords.push((next_x, next_y));
                        grid[next_x as usize][next_y as usize] = 2;
                        fresh_oranges -= 1;
                    }
                }
            }
        }
        if coords.len() != 0 {
            time_to_infect += 1;
        }
    }

    if fresh_oranges == 0 {
        time_to_infect
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn orange_rotting_test() {
        assert_eq!(4, super::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]));
    }
}
