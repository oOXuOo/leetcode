///
/// 以数组 intervals 表示若干个区间的集合，其中单个区间为
/// intervals[i] = [starti, endi] 。请你合并所有重叠的区间，
/// 并返回一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间。
/// 链接：https://leetcode-cn.com/problems/merge-intervals
///
use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let l = intervals.len();
        if l == 1 {return intervals;}
        for _ in 0..l {
            for i in 0..(l-1) {
                if intervals[i][0] == -1 || intervals[i][1] < intervals[i+1][0] {
                    continue;
                }
                if intervals[i][0] > intervals[i+1][1] {
                    let tmp = vec![intervals[i][0], intervals[i][1]];
                    intervals[i] = vec![intervals[i+1][0], intervals[i+1][1]];
                    intervals[i+1] = vec![tmp[0],tmp[1]];
                    continue;
                }
                let tmp = vec![
                    min(intervals[i][0],intervals[i+1][0]),
                    max(intervals[i][1],intervals[i+1][1])];
                intervals[i] = vec![-1,-1];
                intervals[i+1] = vec![tmp[0],tmp[1]];
            }
        }
        intervals.retain(|v| v[0] != -1);
        return intervals;
    }
}

#[test]
fn test() {
    let intervals =
        vec![vec![2, 3], vec![4, 5], vec![6, 7], vec![1,10]];
    let result = Solution::merge(intervals);
    println!("合并后的序列{:?}", result);
}