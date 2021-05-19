use std::collections::HashMap;

///
/// 给定一个整数数组 nums 和一个整数目标值 target，
/// 请你在该数组中找出 和为目标值 的那 两个 整数，并返回它们的数组下标。
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
/// 你可以按任意顺序返回答案。
/// 链接：https://leetcode-cn.com/problems/two-sum
///
pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.len();
        let mut map = HashMap::with_capacity(l);
        for i in 0..l {
            match map.get(&(target-nums[i])) {
                None => { map.insert(nums[i], i);},
                Some(index) => return vec![*index as i32, i as i32]
            }
        }
        return vec![0, 0]
    }
}

#[test]
fn test() {
    let nums = vec![2,7,11,15];
    println!("两个数的下标是：{:?}", Solution::two_sum(nums, 9));
}