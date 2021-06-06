use std::cmp::max;

///
/// 给定一个整数数组 nums ，找到一个具有最大和的连续子数组
/// （子数组最少包含一个元素），返回其最大和。
/// 链接：https://leetcode-cn.com/problems/maximum-subarray
///
pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre = nums[0];
        let mut result = pre;
        for index in 1..nums.len() {
            let curr = max(nums[index], pre + nums[index]);
            result = max(result, curr);
            pre = curr;
        }
        return result;
    }
}

#[test]
fn test() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("最大连续子序列和为：{}", Solution::max_sub_array(nums));
}