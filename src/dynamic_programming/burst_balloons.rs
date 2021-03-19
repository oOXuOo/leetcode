///
/// 有 n 个气球，编号为0 到 n - 1，每个气球上都标有一个数字，
/// 这些数字存在数组nums中。
/// 现在要求你戳破所有的气球。戳破第 i 个气球，
/// 你可以获得nums[i - 1] * nums[i] * nums[i + 1] 枚硬币。
/// 这里的 i - 1 和 i + 1 代表和i相邻的两个气球的序号。
/// 如果 i - 1或 i + 1 超出了数组的边界，那么就当它是一个数字为 1 的气球。
/// 求所能获得硬币的最大数量。
/// 链接：https://leetcode-cn.com/problems/burst-balloons
///

pub struct Solution {}

impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        nums.insert(0, 1);
        nums.push(1);
        let sub_map = &mut vec![vec![-1;nums.len()];nums.len()];
        return get_max(&nums, sub_map, 0, nums.len()-1);
    }
}

fn get_max(nums: &Vec<i32>, sub_map: &mut Vec<Vec<i32>>,
           i: usize, j: usize) -> i32 {
    if i + 1 == j { return 0;}
    if i + 2 == j { return nums[i] * nums[i+1] * nums[j];}
    let mut max = -999;
    for index in i+1..j {
        let tmp = cache_get_sub(nums, sub_map, i, index) +
                cache_get_sub(nums, sub_map, index, j) +
            nums[i] * nums[index] * nums[j];
        if max < tmp { max = tmp}
    }
    return max;
}

fn cache_get_sub(nums: &Vec<i32>, sub_map: &mut Vec<Vec<i32>>,
                 i: usize, j: usize) -> i32 {
    let mut max = sub_map[i][j];
    if max == -1 {
        max = get_max(nums, sub_map, i, j);
        sub_map[i][j] = max;
    };
    return max;
}

#[test]
fn test() {
    let nums= vec![3,1,5,8];
    let max = Solution::max_coins(nums);
    println!("最大得分为{}", max)
}