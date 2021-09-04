use std::cmp;

/// https://leetcode.com/problems/longest-substring-without-repeating-characters/
#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    fn longest_of_substring_internal(s: &str) -> i32 {
        let mut marker: i128 = 0;
        let mut max_len = 0;
        let mut current_len = 0;
        for c in s.bytes() {
            if marker & (1 << c) == 0 {
                current_len += 1;
                marker |= 1 << c;
            } else {
                max_len = cmp::max(current_len, max_len);
                current_len = 1;
                marker |= 1 << c;
            }
        }
        if max_len == 0 {
            s.len() as i32
        } else {
            cmp::max(max_len, current_len)
        }
    }
    let mut max = 0;
    for i in 0..s.len() {
        max = cmp::max(max, longest_of_substring_internal(&s.as_str()[i..]));
    }
    max
}

#[allow(dead_code)]
pub fn length_of_longest_substring_2(s: String) -> i32 {
    let mut marker_arr = [-1i32; 256];
    let mut iterator = 0i32;
    let mut max_len = 0;

    if s.len() == 1 {
        return 1;
    } else if s.len() == 0 {
        return 0;
    }

    for (idx, c) in s.chars().enumerate() {
        let prev_position = marker_arr[c as usize];
        iterator = cmp::max(iterator, prev_position + 1);
        max_len = cmp::max(max_len, idx as i32 - iterator + 1);
        marker_arr[c as usize] = idx as i32;
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn length_of_longest_substring_test() {
        assert_eq!(
            4,
            super::length_of_longest_substring("adfsfdsdfaaaaddd".to_string())
        );
        assert_eq!(1, super::length_of_longest_substring(" ".to_string()));
        assert_eq!(2, super::length_of_longest_substring("aab".to_string()));
        assert_eq!(3, super::length_of_longest_substring("dvdf".to_string()));
    }

    #[test]
    fn length_of_longest_substring_2_test() {
        assert_eq!(
            4,
            super::length_of_longest_substring_2("adfsfdsdfaaaaddd".to_string())
        );
        assert_eq!(1, super::length_of_longest_substring_2(" ".to_string()));
        assert_eq!(2, super::length_of_longest_substring_2("aab".to_string()));
        assert_eq!(3, super::length_of_longest_substring_2("dvdf".to_string()));

        assert_eq!(1, super::length_of_longest_substring_2("bbbb".to_string()));
        assert_eq!(2, super::length_of_longest_substring_2("au".to_string()));
        assert_eq!(
            5,
            super::length_of_longest_substring_2("anviaj".to_string())
        );
    }
}
