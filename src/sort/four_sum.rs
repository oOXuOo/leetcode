///
/// 给定一个包含 n 个整数的数组 nums 和一个目标值 target，
/// 判断 nums 中是否存在四个元素 a，b，c 和 d，使得 a + b + c + d = target
/// 找出所有满足条件且不重复的四元组。
///
/// 注意：答案中不可以包含重复的四元组
/// 链接：https://leetcode-cn.com/problems/4sum
///
pub struct Solution {}

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if nums.len() < 4 { return result;}
        sort_and_uniq(&mut nums, 4);
        for i in 0..nums.len() {
            let a = nums[i];
            if a > 0 && a > target { break; }
            if i > 0 && nums[i] == nums[i-1] {continue};
            for j in i + 1..nums.len() {
                let b = nums[j];
                if b > 0 && a + b > target { break; }
                if j > i+1 && nums[j] == nums[j-1] {continue};
                for k in j + 1..nums.len() {
                    let c = nums[k];
                    if c > 0 && a + b + c > target { break; }
                    if k > j+1 && nums[k] == nums[k-1] {continue};
                    for l in k + 1..nums.len() {
                        let d = nums[l];
                        if a + b + c + d > target { break; }
                        if l > k+1 && nums[l] == nums[l-1] {continue};
                        if a + b + c + d == target {
                            result.push(vec![a, b, c, d]);
                        };
                    }
                }
            }
        }
        return result;
    }
}

fn sort_and_uniq(nums: &mut Vec<i32>, n: usize) {
    nums.sort();
    for index in (0..=nums.len() - 1).rev() {
        if index >= n && nums[index-n] == nums[index] {
            nums.remove(index);
        }
    }
}

#[test]
fn test() {
    let nums = vec![1,-2,-5,-4,-3,3,3,5];
    let target = -11;
    let result = Solution::four_sum(nums, target);
    println!("满足条件的四元组为：{:?}", result)
}