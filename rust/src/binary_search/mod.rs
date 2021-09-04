/// https://leetcode.com/problems/binary-search/
#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() > 0 {
        if nums.len() == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }

        let idx = nums.len() / 2;
        if nums[idx] == target {
            return idx as i32;
        } else if nums[idx] < target {
            return match search(nums[idx..].to_vec(), target) {
                -1 => -1,
                i => i + idx as i32,
            };
        } else {
            return search(nums[0..idx].to_vec(), target);
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_search_test() {
        assert_eq!(-1, super::search(vec![-1, 0, 3, 5, 9, 12], 2));
        assert_eq!(4, super::search(vec![-1, 0, 3, 5, 9, 12], 9));
    }
}
