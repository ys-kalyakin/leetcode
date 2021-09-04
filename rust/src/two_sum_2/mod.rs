/// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

#[allow(dead_code)]
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, v) in numbers.iter().enumerate() {
        if let Ok(j) = numbers[i..].binary_search(&(target - v)) {
            return vec![i as i32 + 1, (j + i) as i32 + 1];
        }
    }
    vec![0, 0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_sum_2_test() {
        assert_eq!(vec![1, 2], super::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 3], super::two_sum(vec![2, 3, 4], 6));
        assert_eq!(vec![1, 2], super::two_sum(vec![-1, 0], -1));
        assert_eq!(vec![2, 3], super::two_sum(vec![5, 25, 75], 100));
        assert_eq!(vec![2, 3], super::two_sum(vec![35, 50, 50], 100));
    }
}
