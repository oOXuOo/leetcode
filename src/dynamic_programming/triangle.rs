use std::collections::HashMap;

///
/// 给定一个三角形 triangle ，找出自顶向下的最小路径和。
/// 每一步只能移动到下一行中相邻的结点上。
/// 相邻的结点 在这里指的是 下标 与 上一层结点下标 相同或者
/// 等于 上一层结点下标 + 1 的两个结点。也就是说，
/// 如果正位于当前行的下标 i ，那么下一步可以移动到下一行的下标 i 或 i + 1 。
/// 链接：https://leetcode-cn.com/problems/triangle
///

pub struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let line_index = triangle.len() - 1;
        let bottom = &triangle[line_index];

        let sub_map = &mut HashMap::new();
        sub_map.insert("0:0", triangle[0][0]);
        if triangle.len() == 1 {
            return triangle[0][0];
        };

        let mut min = None;
        for (index,value_ref) in bottom.iter().enumerate() {
            if index != line_index { // 不是最后一个
                let v = *value_ref +
                    cache_i_j(&triangle, sub_map, line_index - 1, index);
                if min.is_none() || min.unwrap() > v { min = Some(v); }
            }
            if index != 0 { // 不是第一个
                let v = *value_ref +
                    cache_i_j(&triangle, sub_map, line_index - 1, index -1);
                if min.is_none() || min.unwrap() > v { min = Some(v); }
            }
        };
        return min.unwrap();
    }
}

fn cache_i_j(triangle: &Vec<Vec<i32>>,
             sub_map: &mut HashMap<&str, i32>,
             i: usize, j: usize) -> i32 {
    let s = format!("{}:{}", i, j);
    let k = Box::leak(s.into_boxed_str());
    let opt = sub_map.get(k);
    let mut sub_min = None;
    if opt.is_some() {
        sub_min = Some(*opt.unwrap());
    } else {
        if j != i { // 不是最后一个
            let tmp = triangle[i][j] +
                cache_i_j(triangle, sub_map, i - 1, j);
            if sub_min.is_none() {
                sub_min = Some(tmp);
            }
        }
        if j != 0 { // 不是第一个
            let tmp = triangle[i][j] +
                cache_i_j(triangle, sub_map, i - 1, j - 1);
            if sub_min.is_none() {
                sub_min = Some(tmp);
            } else if sub_min.unwrap() > tmp {
                sub_min = Some(tmp);
            }
        }
    };
    sub_map.insert(k, sub_min.unwrap());
    return sub_min.unwrap();
}

#[test]
fn test() {
    let mut triangle = Vec::new();
    triangle.push(vec![3]);
    triangle.push(vec![5, 6]);
    triangle.push(vec![84, 45, 44]);
    triangle.push(vec![2, 4, 87, 9]);
    let total = Solution::minimum_total(triangle);
    println!("最短路径和是：{}", total);
}