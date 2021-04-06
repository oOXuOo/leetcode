///
/// 给定一个只包含数字的字符串，用以表示一个 IP 地址，返回所有可能
/// 从 s 获得的 有效 IP 地址 。你可以按任何顺序返回答案。
///
/// 有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有前导 0），
/// 整数之间用 '.' 分隔。
///
/// 例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，
/// 但是 "0.011.255.245"、"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。
/// 链接：https://leetcode-cn.com/problems/restore-ip-addresses
///
pub struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let l = s.len();
        let mut result = vec![];
        if l < 4 || l > 12 { return result;}
        let p0 = 0;
        let mut p1 = p0 + 1;
        while p1 <= l {
            if p1 - p0 > 3 { break }
            if p1 - p0 > 1 && &s[p0..=p0] == "0" { break }
            if p1 - p0 == 3 && &s[p0..p1] > "255" { break }
            if l - p1 < 3 { break }
            if l - p1 > 9 { p1 += 1; continue}
            let mut p2 = p1 + 1;
            while p2 <= l {
                if p2 - p1 > 3 { break }
                if p2 - p1 > 1 && &s[p1..=p1] == "0" { break }
                if p2 - p1 == 3 && &s[p1..p2] > "255" { break }
                if l - p2 < 2 { break }
                if l - p2 > 6 { p2 += 1; continue}
                let mut p3 = p2 + 1;
                while p3 <= l {
                    if p3 - p2 > 3 { break }
                    if p3 - p2 > 1 && &s[p2..=p2] == "0" { break }
                    if p3 - p2 == 3 && &s[p2..p3] > "255" { break }
                    if l - p3 < 1 { break }
                    if l - p3 > 3 { p3 += 1; continue}
                    if p3 + 1 != l && &s[p3..=p3] == "0" { p3 += 1; continue }
                    if l - p3 == 3 && &s[p3..] > "255" { p3 += 1; continue }
                    let ip = format!(
                        "{}.{}.{}.{}", &s[..p1], &s[p1..p2], &s[p2..p3], &s[p3..]);
                    result.push(ip);
                    p3 += 1;
                }
                p2 += 1;
            }
            p1 += 1;
        }
        return result;
    }
}

#[test]
fn test() {
    let s = String::from("25525511135");
    let v = Solution::restore_ip_addresses(s);
    println!("有效ip有{:?}", v);
}