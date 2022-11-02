/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::{collections::HashMap, iter::FromIterator};

// a + b = target -> b = target - a
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map: HashMap<i32, usize> = nums
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, v)| (v, idx))
            .collect();

        let mut res: Vec<i32> = Vec::new();
        for (idx, num) in nums.into_iter().enumerate() {
            if let Some(second_idx) = map.get(&(num - target).abs()) {
                res = vec![idx as i32, (*second_idx) as i32];
                break;
            }
        }

        res
    }
}
// @lc code=end
