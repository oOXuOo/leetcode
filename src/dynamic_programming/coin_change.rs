use std::collections::HashMap;

///
/// 给定不同面额的硬币 coins 和一个总金额 amount。
/// 编写一个函数来计算可以凑成总金额所需的最少的硬币个数。
/// 如果没有任何一种硬币组合能组成总金额，返回 -1;
/// 你可以认为每种硬币的数量是无限的。
/// 链接：https://leetcode-cn.com/problems/coin-change
///
pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let v = &coins;
        let mut sub_map = HashMap::new();
        sub_map.insert(0, 0);
        return inner(v, amount, &mut sub_map);
    }
}

fn inner(coins: &Vec<i32>, amount: i32,
         sub_map: &mut HashMap<i32, i32>) -> i32 {
    if sub_map.contains_key(&amount) {
        return *(sub_map.get(&amount).unwrap());
    }
    let mut min = -1;
    for coin_ref in coins {
        let coin = *coin_ref;
        if coin > amount {
            continue;
        } else if coin == amount {
            return 1;
        } else {
            let sub_amount = amount - coin;
            let opt = sub_map.get(&sub_amount);
            let sub_min;
            if opt.is_some() {
                sub_min = *(opt.unwrap());
            } else {
                sub_min = inner(coins, sub_amount, sub_map);
            }
            sub_map.insert(sub_amount, sub_min);
            if (sub_min > 0) && (min < 0 || min > (sub_min + 1)) {
                min = sub_min + 1;
            }
        };
    };
    return min;
}

#[test]
fn test() {
    let coins = vec![186,419,83,408];
    let answer = Solution::coin_change(coins, 6249);
    println!("所需最小硬币数为{}", answer);
}