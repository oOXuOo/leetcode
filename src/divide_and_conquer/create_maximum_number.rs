///
/// 给定长度分别为m和n的两个数组，其元素由0-9构成，
/// 表示两个自然数各位上的数字。现在从这两个数组中选出
/// k (k <= m + n) 个数字拼接成一个新的数，要求:
/// 从同一个数组中取出的数字保持其在原数组中的相对顺序。
///
/// 求满足该条件的最大数。结果返回一个表示该最大数的长度为k的数组。
/// 说明: 请尽可能地优化你算法的时间和空间复杂度。
/// 链接：https://leetcode-cn.com/problems/create-maximum-number
///

pub struct Solution {}

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let x = k as usize;
        let mut max = vec![0; x];
        for n in 0..=x {
            if n > nums1.len() { break; }
            if x - n > nums2.len() { continue; }
            let v1 = get_max_range(&nums1, n);
            let v2 = get_max_range(&nums2, x - n);
            let tmp = merge_max_range(&v1, &v2);
            if vector_compare(&tmp, &max) > 0 {
                max = tmp;
            }
        };
        return max;
    }
}

fn get_max_range(nums: &Vec<i32>, cnt: usize) -> Vec<i32> {
    if cnt == 0 { return Vec::new(); };
    let mut result = vec![0; cnt];
    let mut max = 0 as usize;
    let mut stop = nums.len() - cnt;
    for i in 0..cnt {
        let range = max..=stop;
        for index in range {
            if nums[index] > nums[max] {
                max = index;
            };
        }
        result[i] = nums[max];
        max += 1;
        stop += 1;
    };
    return result;
}

fn merge_max_range(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut i = 0 as usize;
    let mut j = 0 as usize;
    while i < v1.len() && j < v2.len() {
        if vector_compare(&v1[i..], &v2[j..]) > 0 {
            result.push(v1[i]);
            i += 1;
        } else {
            result.push(v2[j]);
            j += 1;
        }
    };
    while i < v1.len() {
        result.push(v1[i]);
        i += 1;
    }
    while j < v2.len() {
        result.push(v2[j]);
        j += 1;
    }
    return result;
}

fn vector_compare(v1: &[i32], v2: &[i32]) -> i32 {
    let v = if v1.len() > v2.len() { v2} else { v1 };
    for (i, _) in v.iter().enumerate() {
        if v1[i] > v2[i] { return 1; }
        if v1[i] < v2[i] { return -1; }
    };
    return if v1.len() > v2.len() { 1 }
    else if v1.len() < v2.len() { -1 }
    else {0};
}

#[test]
fn test() {
    let nums1 = vec![0,2,0,1];
    let nums2 = vec![0,2,0,1];
    let result = Solution::max_number(nums1, nums2, 8);
    println!("最大数为{:?}", result)
}