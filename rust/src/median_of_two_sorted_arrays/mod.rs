/// https://leetcode.com/problems/median-of-two-sorted-arrays/
#[allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let merged_array: Vec<i32> = merge(&nums1, &nums2);
    if merged_array.len() == 0 {
        return 0.0;
    } else if merged_array.len() == 1 {
        return merged_array[0] as f64;
    }

    if merged_array.len() % 2 != 0 {
        return merged_array[merged_array.len() / 2] as f64;
    } else {
        return (merged_array[merged_array.len() / 2] as f64
            + merged_array[merged_array.len() / 2 - 1] as f64)
            / 2.0;
    }
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut merged = vec![];

    let (mut ai, mut bi) = (0, 0);

    while ai < a.len() && bi < b.len() {
        let (x, y) = (a[ai], b[bi]);
        if x < y {
            merged.push(x);
            ai += 1;
        } else {
            merged.push(y);
            bi += 1;
        }
    }
    merged.extend(if ai < a.len() { &a[ai..] } else { &b[bi..] });
    merged
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_median_sorted_arrays_test() {
        assert_eq!(2.0, super::find_median_sorted_arrays(vec![1, 3], vec![2]));
        assert_eq!(
            2.5,
            super::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );
        assert_eq!(1.0, super::find_median_sorted_arrays(vec![], vec![1]));
    }
}
