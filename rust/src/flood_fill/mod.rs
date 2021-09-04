#[allow(dead_code)]
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let color = image[sr as usize][sc as usize];
    if color == new_color {
        return image;
    }

    let mut result = image;
    result[sr as usize][sc as usize] = new_color;
    if sr as usize + 1 < result.len() && result[sr as usize + 1][sc as usize] == color {
        result = flood_fill(result, sr + 1, sc, new_color);
    }
    #[allow(unused_comparisons)]
    if sr > 0 && sr as usize - 1 >= 0 && result[sr as usize - 1][sc as usize] == color {
        result = flood_fill(result, sr - 1, sc, new_color);
    }
    if sc as usize + 1 < result[sr as usize].len() && result[sr as usize][sc as usize + 1] == color
    {
        result = flood_fill(result, sr, sc + 1, new_color);
    }
    #[allow(unused_comparisons)]
    if sc > 0 && sc as usize - 1 >= 0 && result[sr as usize][sc as usize - 1] == color {
        result = flood_fill(result, sr, sc - 1, new_color);
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn flood_fill_test() {
        assert_eq!(
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
            super::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2)
        );
    }
}
