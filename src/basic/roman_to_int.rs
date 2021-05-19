///
/// 罗马数字转整数
/// 给定一个罗马数字，将其转换成整数。输入确保在 1 到 3999 的范围内。
/// 链接：https://leetcode-cn.com/problems/roman-to-integer
///
pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.bytes().rev().fold((0, 0), |res, cur| {
            let n = match cur {
                b'I' => 1,   b'V' => 5,   b'X' => 10,   b'L' => 50,
                b'C' => 100, b'D' => 500, b'M' => 1000, _ => -9999
            };
            (if n < res.1 { res.0 - n } else { res.0 + n }, n)
        }).0
    }
}

#[test]
fn test() {
    let s = String::from("MCMXCIV");
    println!("整数是：{}", Solution::roman_to_int(s));
}