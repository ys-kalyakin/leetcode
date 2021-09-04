/// https://leetcode.com/problems/reverse-words-in-a-string-iii/

#[allow(dead_code)]
pub fn reverse_words(s: String) -> String {
    let mut result = String::new();
    let mut word_start = 0;
    for (i, c) in s.chars().enumerate() {
        if c == ' ' || i == s.len() - 1 {
            let mut substr: Vec<char> = (if i == s.len() - 1 {
                &s[word_start..=i]
            } else {
                &s[word_start..i]
            })
            .chars()
            .collect();
            substr.reverse();
            result.push_str(&substr.into_iter().collect::<String>()[0..]);
            if i != s.len() - 1 {
                result.push(' ');
                word_start = i + 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn reverse_words_test() {
        assert_eq!(
            "s'teL ekat edoCteeL tsetnoc",
            super::reverse_words("Let's take LeetCode contest".to_string())
        );
    }
}
