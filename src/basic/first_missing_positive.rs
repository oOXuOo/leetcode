///
/// 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
/// 进阶：你可以实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案吗？
/// 链接：https://leetcode-cn.com/problems/first-missing-positive
///
pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        for num in &nums {
            if *num > 0 && *num < min {
                min = *num;
            }
        }
        if min > 1 { return 1; }
        let mut v = vec![0; nums.len()];
        for num in &nums {
            if *num <= 0 {continue}
            let index = (*num - min) as usize;
            if index < v.len() { v[index] = 1;}
        }
        let mut index = 0;
        while index < v.len() {
            if v[index] == 0 { break }
            index += 1;
        };
        return min + index as i32;
    }
}


#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, -5];
    let result = Solution::first_missing_positive(nums);
    println!("缺失的最小正整数为{}", result);
}