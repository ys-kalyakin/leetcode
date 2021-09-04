/// https://leetcode.com/problems/reverse-string/

#[allow(dead_code)]
pub fn reverse_string(s: &mut Vec<char>) {
    s.reverse()
}

#[allow(dead_code)]
pub fn reverse_string_2(s: &mut Vec<char>) {
    let mut i = 0;
    let mut j = s.len() - 1;

    while i < j {
        let tmp = s[i];
        s[i] = s[j];
        s[j] = tmp;
        i += 1;
        j -= 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reverse_string_test() {
        let mut v = vec!['h', 'e', 'l', 'l', 'o'];
        super::reverse_string(&mut v);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], v);
    }

    #[test]
    fn reverse_string_2_test() {
        let mut v = vec!['h', 'e', 'l', 'l', 'o'];
        super::reverse_string_2(&mut v);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], v);
        let mut v1 = vec!['h', 'e', 'l', 'l', 'o', 'w'];
        super::reverse_string_2(&mut v1);
        assert_eq!(vec!['w', 'o', 'l', 'l', 'e', 'h'], v1);
    }
}
