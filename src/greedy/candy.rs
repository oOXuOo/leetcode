///
/// 老师想给孩子们分发糖果，有 N 个孩子站成了一条直线，
/// 老师会根据每个孩子的表现，预先给他们评分。
/// 你需要按照以下要求，帮助老师给这些孩子分发糖果：
/// 每个孩子至少分配到 1 个糖果。
/// 评分更高的孩子必须比他两侧的邻位孩子获得更多的糖果。
/// 那么这样下来，老师至少需要准备多少颗糖果呢？
///
/// 输入：[1,2,2] 输出：4
/// 解释：你可以分别给这三个孩子分发 1、2、1 颗糖果。
/// 第三个孩子只得到 1 颗糖果，这已满足上述两个条件。
/// 链接：https://leetcode-cn.com/problems/candy
///
pub struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let l = ratings.len();
        if l <= 1 { return l as i32; }
        let mut v = vec![1; l];
        for index in 1..l {
            if ratings[index] > ratings[index - 1] {
                v[index] = v[index - 1] + 1;
            } else if ratings[index] == ratings[index - 1] {
                v[index] = 1;
            } else {
                v[index] = 1;
                if v[index - 1] - 1 < 1 {
                    back_trace(&ratings, &mut v, index - 1)
                };
            };
        }
        return v.iter().sum();
    }
}

fn back_trace(ratings: &Vec<i32>, v: &mut Vec<i32>, index: usize) {
    for i in (0..=index).rev() {
        if ratings[i] > ratings[i+1] && v[i] == v[i+1] {
            v[i] += 1;
        } else { break }
    }
}

#[test]
fn test() {
    let ratings = vec![1,2,3,1,0];
    println!("最少需要{}块糖果", Solution::candy(ratings));
}