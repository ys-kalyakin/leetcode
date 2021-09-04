/// https://leetcode.com/problems/first-bad-version/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    #[allow(warnings)]
    pub fn isBadVersion(&self, version: i32) -> bool {
        version == 4
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;

        while l != r {
            let middle = l + (r - l) / 2;
            if self.isBadVersion(middle) {
                r = middle;
            } else {
                l = middle + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bad_version_test() {
        assert_eq!(4, super::Solution {}.first_bad_version(5));
    }
}
