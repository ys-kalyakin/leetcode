/// https://leetcode.com/problems/search-insert-position/

#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() > 0 {
        if nums.len() == 1 {
            return if nums[0] == target {0} else {if nums[0] >= target {0} else {1}};
        }

        let idx = nums.len()/2;
        if nums[idx] == target {
            return idx as i32;
        } else if nums[idx] < target {
            return match search_insert(nums[idx..].to_vec(), target) {
                -1 => -1, // not reachable??
                i => i + idx as i32
            };
        } else {
            return search_insert(nums[0..idx].to_vec(), target);
        }
    } 
    if nums[0] >= target {0} else {nums.len() as i32}
}


#[cfg(test)]
mod tests {
    #[test]
    fn search_insrt_test() {
        assert_eq!(2, super::search_insert(vec![1, 3, 5, 6], 5));
        assert_eq!(1, super::search_insert(vec![1, 3, 5, 6], 2));
        assert_eq!(4, super::search_insert(vec![1, 3, 5, 6], 7));
        assert_eq!(0, super::search_insert(vec![1, 3, 5, 6], 0));
        assert_eq!(0, super::search_insert(vec![1], 0));
    }
}
