use std::collections::HashMap;

///
/// 给你一个字符串 s 和一个整数 k ，
/// 请你找出 s 中的最长子串，
/// 要求该子串中的每一字符出现次数都不少于 k 。
/// 返回这一子串的长度。
///
pub struct Solution {}

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        if k == 1 { return s.len() as i32 }
        let v = s.chars().collect();
        return inner(&v, k);
    }
}

fn inner(str: &Vec<char>, k: i32) -> i32 {
    if str.len() < k as usize { return 0 }
    let m = &mut HashMap::new();
    let mut max = 0;
    for c in str.iter() {
        let cnt = match m.get(c) {
            None => 1,
            Some(x) => x + 1
        };
        m.insert(c, cnt);
    }
    let sub_v = &mut Vec::new();
    let mut start = 0;
    for (i, c) in str.iter().enumerate() {
        let opt = m.get(c);
        if opt.is_some() && *opt.unwrap() < k {
            sub_v.push((start, i));
            start = i + 1;
        };
    }
    if sub_v.is_empty() { return str.len() as i32;}
    if start < str.len() {sub_v.push((start, str.len()))}
    for t in sub_v.iter() {
        let sub_str = str[t.0..t.1].to_vec();
        if sub_str.len() < k as usize { continue }
        let tmp = inner(&sub_str, k);
        max = if tmp > max {tmp} else {max};
    }
    return max;
}

#[test]
fn test() {
    let s = String::from("aaabb");
    let x = Solution::longest_substring(s, 3);
    println!("最长子串长度{}", x)
}