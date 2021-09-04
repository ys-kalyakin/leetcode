/// https://leetcode.com/problems/permutations/

#[allow(dead_code)]
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn permute_internal(vec: &mut Vec<Vec<i32>>, size: i32, temp: &mut Vec<i32>, nums: &Vec<i32>) {
        if temp.len() == size as usize {
            vec.push(temp.to_vec());
        } else {
            for (i, &v) in nums.iter().enumerate() {
                temp.push(v);
                let mut tmp = nums.clone();
                tmp.remove(i);
                permute_internal(vec, size, temp, &tmp);
                temp.pop();
            }
        }
    }

    let mut r = Vec::new();
    let mut temp = Vec::new();
    permute_internal(&mut r, nums.len() as i32, &mut temp, &nums);

    r
}

#[cfg(test)]
mod tests {
    #[test]
    fn permute_test() {
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
            super::permute(vec![1, 2, 3])
        );
    }
}
