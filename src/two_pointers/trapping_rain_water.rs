use std::cmp::min;

///
/// 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，
/// 计算按此排列的柱子，下雨之后能接多少雨水。
///
/// ![img](url:https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/22/rainwatertrap.png)
///
/// 链接: https://leetcode-cn.com/problems/trapping-rain-water/
pub struct Solution {}

impl Solution {
    pub fn trap(v: Vec<i32>) -> i32 {
        let l = v.len();
        if l <= 2 { return 0;}
        return inner_trap(&v, 0, l - 1);
    }
}

fn inner_trap(v: &Vec<i32>, mut left: usize, mut right: usize) -> i32 {
    while left < right && v[left+1] >= v[left] { left += 1 }
    while right > left && v[right-1] >= v[right] { right -= 1 }
    if right - left <= 1 { return 0; }
    let short = min(v[left], v[right]);
    let mut max_index = 0;
    for i in left + 1..=right - 1 {
        if v[i] > short {
            if max_index == 0 || v[i] > v[max_index] {
                max_index = i;
            }
        }
    }
    return if max_index == 0 {
        calculate(v, left, right)
    } else {
        inner_trap(v, left, max_index) + inner_trap(v, max_index, right)
    };
}

fn calculate(v: &Vec<i32>, left: usize, right: usize) -> i32{
    let w = (right - left + 1) as i32;
    let h = min(v[left], v[right]);
    let mut sum = h * 2;
    for i in left + 1..=right - 1 {
        sum += v[i];
    }
    return h * w - sum;
}

#[test]
fn test() {
    let v = vec![4,2,0,3,2,5];
    println!("最多接{}水", Solution::trap(v));
}