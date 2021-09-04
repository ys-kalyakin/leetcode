/// https://leetcode.com/problems/letter-case-permutation/

#[allow(dead_code)]
pub fn letter_case_permutation(s: String) -> Vec<String> {
    fn permutation_inner(temp: &mut Vec<char>, v: &mut Vec<String>, i: usize) {
        if temp.len() == i {
            v.push(temp.iter().collect());
        } else {
            permutation_inner(temp, v, i + 1);
            if temp[i].is_alphabetic() {
                temp[i] = if temp[i].is_ascii_lowercase() {
                    temp[i].to_ascii_uppercase()
                } else {
                    temp[i].to_ascii_lowercase()
                };
                permutation_inner(temp, v, i + 1);
                temp[i] = if temp[i].is_ascii_lowercase() {
                    temp[i].to_ascii_uppercase()
                } else {
                    temp[i].to_ascii_lowercase()
                };
            }
        }
    }

    let mut result = Vec::new();
    let mut temp = s.chars().collect();
    permutation_inner(&mut temp, &mut result, 0);
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn letter_case_permutation_test() {
        assert_eq!(
            vec!["a1b2", "a1B2", "A1b2", "A1B2"],
            super::letter_case_permutation("a1b2".to_string())
        );
        assert_eq!(
            vec!["C", "c"],
            super::letter_case_permutation("C".to_string())
        );
    }
}
