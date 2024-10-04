use std::collections::HashMap;
use std::vec::Vec;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            map.insert(*v, i as i32);
        }

        for (i, v) in nums.iter().enumerate() {
            let i = i as i32;
            let v = *v;
            let diff = target - v;
            if let Some(j) = map.get(&diff) {
                let j = *j;
                if i != j {
                    return vec![i, j];
                }
            }
        }
        vec![-1, -1]
    }
}

fn main() {
    let nums = vec![3, 2, 4];
    let sol = Solution::two_sum(nums, 6);
    println!("Sol: {:?}", sol);
}
