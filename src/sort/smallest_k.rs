///
/// 设计一个算法，找出数组中最小的k个数。以任意顺序返回这k个数均可。
/// 链接：https://leetcode-cn.com/problems/smallest-k-lcci/
///
pub struct Solution {}

impl Solution {
    pub fn smallest_k(mut arr: Vec<i32>, k1: i32) -> Vec<i32> {
        if arr.len() <= 1 {return arr;}
        let k = k1 as usize;
        build_big_heap(&mut arr[0..k]);
        for index in k..arr.len() {
            if arr[index] < arr[0] {
                swap(&mut arr[0..], 0, index);
                heapify(&mut arr[0..k], 0);
            }
        }
        return arr[0..k].to_vec();
    }
}

fn build_big_heap(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }
    let mut index = nums.len() as i32 / 2 + 1;
    while index >= 0 { heapify(nums, index as usize); index -= 1;}
}

fn heapify(nums: &mut [i32], root: usize) {
    let l = root * 2 + 1;
    let r = root * 2 + 2;
    let mut max_index = root;
    if l < nums.len() && nums[l] > nums[max_index] { max_index = l; }
    if r < nums.len() && nums[r] > nums[max_index] { max_index = r; }
    if max_index == root { return; }
    swap(nums, root, max_index);
    heapify(nums, max_index);
}

fn swap(nums: &mut [i32], i: usize, j: usize) {
    let tmp = nums[i];
    nums[i] = nums[j];
    nums[j] = tmp;
}

#[test]
fn test() {
    let k = 3;
    let nums = vec![4,6,3,7,9,2];
    let result = Solution::smallest_k(nums, k);
    println!("最小的{}个数是{:?}", k, result);
}