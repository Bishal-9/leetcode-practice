/*

1. Two Sum

https://leetcode.com/problems/two-sum/description/

*/

use std::io::stdin;

#[allow(unused)]
fn main() {
    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();

    _input.remove(0);
    _input.pop();

    let nums: Vec<i32> = _input.split(",").map(|i| i.parse().unwrap()).collect();

    _input = String::new();
    stdin().read_line(&mut _input).unwrap();
    _input = _input.trim().to_string();

    let target = _input.parse::<i32>().unwrap();

    let _result = two_sum(nums, target);
    println!("{:?}", _result);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut _result: Vec<i32> = Vec::new();
    for (index, integer) in nums.iter().enumerate() {
        for i in index + 1..nums.len() {
            if integer + nums[i] == target {
                _result.push(index as i32);
                _result.push(i as i32);
            }
        }
    }

    _result
}

#[cfg(test)]
mod tests {

    use super::two_sum;

    #[test]
    fn test_case_1() {
        let _result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(_result, vec![0, 1]);
    }

    #[test]
    fn test_case_2() {
        let _result = two_sum(vec![3, 2, 4], 6);
        assert_eq!(_result, vec![1, 2]);
    }

    #[test]
    fn test_case_3() {
        let _result = two_sum(vec![3, 3], 6);
        assert_eq!(_result, vec![0, 1]);
    }
}
