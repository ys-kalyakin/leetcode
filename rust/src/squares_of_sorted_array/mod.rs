/// https://leetcode.com/problems/squares-of-a-sorted-array/
#[allow(dead_code)]
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = nums.iter().map(|i| i.pow(2)).collect();

    result.sort();
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn sorted_squares_test() {
        assert_eq!(
            vec![0, 1, 9, 16, 100],
            super::sorted_squares(vec![-4, -1, 0, 3, 10])
        );
    }
}
