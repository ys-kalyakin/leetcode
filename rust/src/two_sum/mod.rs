/// https://leetcode.com/problems/two-sum/
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (idx, i) in nums.iter().enumerate() {
        for (jdx, j) in nums[idx + 1..].iter().enumerate() {
            if i + j == target {
                return vec![idx as i32, (jdx + idx + 1) as i32];
            }
        }
    }
    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_sum_test() {
        assert_eq!(vec![1, 2], super::two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], super::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![0, 1], super::two_sum(vec![3, 3], 6));
    }
}
