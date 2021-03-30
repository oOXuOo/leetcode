///
/// 给你一个包含 n 个整数的数组 nums，
/// 判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0
/// 请你找出所有和为 0 且不重复的三元组。
/// 链接：https://leetcode-cn.com/problems/3sum
///
pub struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let target = 0;
        if nums.len() < 3 { return result;}
        nums.sort();
        for i in 0..nums.len() {
            let a = nums[i];
            if a > 0 && a > target { break; }
            if a < 0 && a + nums[nums.len()-1] + nums[nums.len()-2] < target {continue}
            if i > 0 && nums[i] == nums[i-1] {continue};
            for j in i + 1..nums.len() {
                let b = nums[j];
                if b > 0 && a + b > target { break; }
                if b < 0 && a + b + nums[nums.len()-1] < target {continue}
                if j > i+1 && nums[j] == nums[j-1] {continue};
                for k in j + 1..nums.len() {
                    let c = nums[k];
                    if a + b + c > target { break; }
                    if k > j+1 && nums[k] == nums[k-1] {continue};
                    if a + b + c == target {
                        result.push(vec![a, b, c]);
                    };
                }
            }
        }
        return result;
    }
}

#[test]
fn test() {
    let nums = vec![-1,0,1,2,-1,-4];
    let result = Solution::three_sum(nums);
    println!("满足条件的三元组为：{:?}", result)
}