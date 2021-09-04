#[allow(dead_code)]
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut v = Vec::new();
    for i in 1..=n {
        v.push(i);
    }

    fn combine_internal(
        vec: &mut Vec<Vec<i32>>,
        size: i32,
        temp: &mut Vec<i32>,
        lens: &Vec<i32>,
        start: i32,
        index: i32,
    ) {
        if index == size {
            vec.push(temp.to_vec());
        } else {
            for i in start..lens.len() as i32 {
                if lens.len() as i32 - i + 1 > size - temp.len() as i32 {
                    (*temp)[index as usize] = (*lens)[i as usize];
                    combine_internal(vec, size, temp, lens, i as i32 + 1, index + 1);
                }
            }
        }
    }

    let mut r = Vec::new();
    let mut temp = vec![0; k as usize];
    combine_internal(&mut r, k, &mut temp, &v, 0, 0);

    r
}

#[cfg(test)]
mod tests {
    #[test]
    fn combine_test() {
        assert_eq!(
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ],
            super::combine(4, 2)
        );
    }
}
