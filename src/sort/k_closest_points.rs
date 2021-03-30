// use std::mem::swap;
//
// ///
// /// 我们有一个由平面上的点组成的列表 points。需要从中找出 K 个距离原点 (0, 0) 最近的点。
// /// （这里，平面上两点之间的距离是欧几里德距离。）
// /// 你可以按任何顺序返回答案。除了点坐标的顺序之外，答案确保是唯一的。
// /// 链接：https://leetcode-cn.com/problems/k-closest-points-to-origin
// ///
// pub struct Solution {}
//
// impl Solution {
//     pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
//         return vec![]
//     }
// }
//
// fn heap_sort(nums: &mut Vec<i32>, k: i32) {
//     build_small_heap(&mut nums[0..]);
//     for index in (0..=nums.len() - 1).rev() {
//         swap(&mut nums[0], &mut nums[index]);
//         if nums.len() - index == k as usize { break; }
//         build_small_heap(&mut nums[0..index]);
//     }
// }
//
// fn build_small_heap(nums: &mut [i32]) {
//     if nums.len() <= 1 { return; }
//     let mut index = nums.len() / 2 + 1;
//     while index >= 0 { heapify(nums, index); }
// }
//
// fn heapify(nums: &mut [i32], root: usize) {
//     let l = root * 2 + 1;
//     let r = root * 2 + 2;
//     let mut min_index = root;
//     if l < nums.len() && nums[l] < nums[min_index] { min_index = l; }
//     if r < nums.len() && nums[r] < nums[min_index] { min_index = r; }
//     if min_index == root { return; }
//     swap(&mut nums[root], &mut nums[min_index]);
//     heapify(nums, min_index);
// }
