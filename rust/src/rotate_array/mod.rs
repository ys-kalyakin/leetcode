/// https://leetcode.com/problems/rotate-array/

/// time limit =(
#[allow(dead_code)]
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    if len == 1 {
        return;
    }
    for _ in 0..k as usize % len {
        let mut i = 0;
        let mut prev_value = nums[0];

        while i < len {
            let next_idx = (i + 1) % (len - 1);
            let tmp = nums[next_idx];
            nums[next_idx] = prev_value;
            prev_value = tmp;
            i += 1;
        }
        let tmp = nums[0];
        nums[0] = nums[len - 1];
        nums[len - 1] = tmp;
    }
}

/// cheat (>_<)
#[allow(dead_code)]
pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
    if nums.len() == 1 {
        return;
    }
    let len = nums.len();
    nums.rotate_right(k as usize % len);
}

#[cfg(test)]
mod tests {
    #[test]
    fn rotate_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        super::rotate(&mut v, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], v);

        let mut v2 = vec![1, 2];
        super::rotate(&mut v2, 3);
        assert_eq!(vec![2, 1], v2);
    }

    #[test]
    fn rotate2_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        super::rotate2(&mut v, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], v);

        let mut v2 = vec![1, 2];
        super::rotate2(&mut v2, 3);
        assert_eq!(vec![2, 1], v2);
    }
}
