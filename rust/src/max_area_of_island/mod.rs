use std::cmp::max;
/// https://leetcode.com/problems/max-area-of-island/

#[allow(dead_code)]
pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    fn area(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, a: &mut i32) {
        *a += 1;
        grid[r][c] = 0;
        if r + 1 < grid.len() && grid[r + 1][c] == 1 {
            area(grid, r + 1, c, a);
        }
        if c + 1 < grid[r].len() && grid[r][c + 1] == 1 {
            area(grid, r, c + 1, a);
        }
        #[allow(unused_comparisons)]
        if r > 0 && r - 1 >= 0 && grid[r - 1][c] == 1 {
            area(grid, r - 1, c, a);
        }
        #[allow(unused_comparisons)]
        if c > 0 && c - 1 >= 0 && grid[r][c - 1] == 1 {
            area(grid, r, c - 1, a);
        }
    }

    let mut result = 0;
    let mut max_a = 0;
    let mut g = grid.clone();
    for (i, v) in grid.iter().enumerate() {
        for (j, value) in v.iter().enumerate() {
            if *value == 1 {
                area(&mut g, i, j, &mut result);
                max_a = max(result, max_a);
                result = 0;
            }
        }
    }

    max_a
}

#[cfg(test)]
mod tests {
    #[test]
    fn max_area_of_island_test() {
        assert_eq!(
            6,
            super::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ])
        );
        assert_eq!(
            4,
            super::max_area_of_island(vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 1, 1]
            ])
        );
        assert_eq!(
            12,
            super::max_area_of_island(vec![
                vec![0, 1, 1, 1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 1, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0, 0, 0, 1],
                vec![0, 0, 1, 0, 1, 1, 0, 1],
                vec![0, 1, 0, 1, 0, 1, 1, 1],
                vec![1, 0, 1, 0, 1, 1, 1, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0]
            ])
        );
    }
}
