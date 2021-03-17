use std::collections::HashMap;

///
/// 给定一个正整数 n，将其拆分为至少两个正整数的和，
/// 并使这些整数的乘积最大化。返回你可以获得的最大乘积。
/// 示例:
/// 输入: 10
/// 输出: 36
/// 解释: 10 = 3 + 3 + 4, 3 ×3 ×4 = 36
/// 说明: 你可以假设 n 不小于 2 且不大于 58
/// 链接：https://leetcode-cn.com/problems/integer-break
///
pub struct Solution {}

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let sub_map = &mut HashMap::new();
        return inner(n, sub_map);
    }
}

fn inner(n: i32, sub_map: &mut HashMap<i32, i32>) -> i32 {
    if n <= 2 { return 1; }
    let mut max = 1;
    let half = n / 2 + 1;
    for part_1 in 1..half {
        let part_2 = n - part_1;

        let opt_1 = sub_map.get(&part_1);
        let sub_product_1;
        if opt_1.is_some() {
            sub_product_1 = *(opt_1.unwrap());
        } else {
            sub_product_1 = inner(part_1, sub_map);
        };
        sub_map.insert(part_1, sub_product_1);

        let opt_2 = sub_map.get(&part_2);
        let sub_product_2;
        if opt_2.is_some() {
            sub_product_2 = *(opt_2.unwrap());
        } else {
            sub_product_2 = inner(part_2, sub_map);
        };
        sub_map.insert(part_2, sub_product_2);

        if part_1 * part_2 > max {
            max = part_1 * part_2;
        }
        if part_1 * sub_product_2 > max {
            max = part_1 * sub_product_2;
        }
        if part_2 * sub_product_1 > max  {
            max = part_2 * sub_product_1;
        }
        if sub_product_1 * sub_product_2 > max {
            max = sub_product_1 * sub_product_2;
        };
    }
    return max;
}

#[test]
fn test() {
    let max = Solution::integer_break(11);
    println!("最大子乘积为{}", max);
}