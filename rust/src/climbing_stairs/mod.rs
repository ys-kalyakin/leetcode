/// https://leetcode.com/problems/climbing-stairs/

#[allow(dead_code)]
pub fn climb_stairs(n: i32) -> i32 {
    let mut v = vec![0; n as usize + 1];
    v[0] = 1;
    v[1] = 1;

    for i in 2..=n as usize {
        v[i] = v[i - 1] + v[i - 2];
    }
    v[n as usize] as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn climb_stairs_test() {
        assert_eq!(2, super::climb_stairs(2));
        assert_eq!(3, super::climb_stairs(3));
    }
}
