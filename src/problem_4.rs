/*

4. Median of Two Sorted Arrays

https://leetcode.com/problems/median-of-two-sorted-arrays/

*/

use std::io::stdin;

#[allow(unused)]
fn main() {
    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();
    _input = (&_input[1.._input.len() - 1]).to_string();
    let nums1: Vec<i32> = _input.split(",").map(|_i| _i.parse().unwrap()).collect();

    _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();
    _input = (&_input[1.._input.len() - 1]).to_string();
    let nums2: Vec<i32> = _input.split(",").map(|_i| _i.parse().unwrap()).collect();
}

fn _calculate_median(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    nums1.extend(nums2);
    nums1.sort();

    let mut _result = 0.0;
    let length = nums1.len();
    if length % 2 == 0 {
        _result =
            (nums1.get(length / 2).unwrap() + nums1.get((length / 2) - 1).unwrap()) as f64 / 2.0;
    } else {
        _result = (*nums1.get(length / 2).unwrap()) as f64;
    }

    _result
}

#[cfg(test)]
mod tests {
    use super::_calculate_median;

    #[test]
    fn test_case_1() {
        let _result = _calculate_median(vec![1, 3], vec![2]);
        assert_eq!(_result, 2.0)
    }

    #[test]
    fn test_case_2() {
        let _result = _calculate_median(vec![1, 2], vec![3, 4]);
        assert_eq!(_result, 2.5)
    }
}
