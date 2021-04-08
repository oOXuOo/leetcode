use std::cmp::max;

///
/// 给你 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点(i,ai) 。
/// 在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0) 。
/// 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
/// 链接：https://leetcode-cn.com/problems/container-with-most-water
///
pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let l = height.len();
        if l <= 1 { return 0; }
        let mut max_area = 0;
        let mut i = 0;
        let mut j = l - 1;
        while i < j {
            let short;
            let width = (j - i) as i32;
            if height[i] < height[j] {
                short = height[i];
                i += 1;
            } else {
                short = height[j];
                j -= 1;
            };
            max_area = max(max_area, width * short);
        };
        return max_area;
    }
}

#[test]
fn test() {
    let v = vec![1,8,6,2,5,4,8,3,7];
    println!("最多容{}水", Solution::max_area(v));
}