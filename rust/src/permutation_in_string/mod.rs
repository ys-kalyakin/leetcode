/// https://leetcode.com/problems/permutation-in-string/

#[allow(dead_code)]
pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s2.len() < s1.len() {
        return false;
    }

    let mut sv: Vec<char> = s1.chars().collect();
    sv.sort();
    let s2v: Vec<char> = s2.chars().collect();
    let mut i = 0;
    while i <= s2.len() - s1.len() {
        let mut t = s2v[i..i + s1.len()].to_vec();
        t.sort();
        println!("{:?}", t);
        if sv == t {
            return true;
        }
        i += 1;
    }
    false
}

/// time limit
#[allow(dead_code)]
pub fn check_inclusion_2(s1: String, s2: String) -> bool {
    fn swap(s: &String, i0: i32, i1: i32) -> String {
        if i0 == i1 {
            return String::from(s);
        }

        let s1 = &s[0..i0 as usize];
        let s2 = &s[i0 as usize + 1..i1 as usize];
        let s3 = &s[i1 as usize + 1..];

        let mut ret = String::new();
        ret.push_str(s1);
        ret.push_str(&s[i1 as usize..i1 as usize + 1]);
        ret.push_str(s2);
        ret.push_str(&s[i0 as usize..i0 as usize + 1]);
        ret.push_str(s3);
        ret
    }

    fn permute(s1: &String, s2: &String, l: usize) -> bool {
        if l == s1.len() {
            return s2.contains(s1);
        } else {
            let mut i = l;
            let mut s = s1;
            let slen = s.len();
            let mut result = false;
            let mut t2;
            while i < slen {
                let t = swap(&s, l as i32, i as i32);
                result |= permute(&t, s2, l + 1);
                t2 = swap(&t, l as i32, i as i32);
                s = &t2;
                i += 1;
            }
            return result;
        }
    }
    if s1.len() > s2.len() {
        return false;
    }
    permute(&s1, &s2, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_inclusion_test() {
        assert!(super::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string()
        ));
        assert!(super::check_inclusion(
            "adc".to_string(),
            "dcda".to_string()
        ));
    }

    #[test]
    fn check_inclusion_2_test() {
        assert!(super::check_inclusion_2(
            "ab".to_string(),
            "eidbaooo".to_string()
        ));
        assert!(super::check_inclusion_2(
            "adc".to_string(),
            "dcda".to_string()
        ));
        assert!(!super::check_inclusion_2(
            "dinitrophenylhydrazine".to_string(),
            "acetylphenylhydrazine".to_string()
        ));
        assert!(!super::check_inclusion_2(
            "acff".to_string(),
            "caaa".to_string()
        ));
    }
}
