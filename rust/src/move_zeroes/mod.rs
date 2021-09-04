/// https://leetcode.com/problems/move-zeroes/

#[allow(dead_code)]
pub fn move_zeroes(nums: &mut Vec<i32>) {
    if nums.len() == 1 {
        return;
    }

    let mut i = 0;
    let len = nums.len();
    while i < len {
        if nums[i] == 0 {
            for j in i..len {
                if nums[j] != 0 {
                    nums[i] = nums[j];
                    nums[j] = 0;
                    break;
                }
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_zeroes_test() {
        let mut v = vec![0, 1, 0, 3, 12];
        super::move_zeroes(&mut v);
        assert_eq!(vec![1, 3, 12, 0, 0], v);
    }
}
