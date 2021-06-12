use std::collections::HashMap;
use std::cmp::max;

pub struct Solution {}
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();
        let mut cache = HashMap::new();
        let max_half = get_max(sum / 2, stones.len() - 1, &stones, &mut cache);
        println!("缓存项：{:?}", cache);
        return sum - max_half * 2;
    }
}

fn get_max(top_v: i32, top_i: usize, stones: &Vec<i32>,
           cache: &mut HashMap<String, i32>) -> i32 {
    if top_i == 0 {
        return if stones[0] > top_v {0} else {stones[0]};
    }
    let k = format!("{}_{}", top_i, top_v);
    if cache.contains_key(&k) {
        println!("缓存命中");
        return *cache.get(&k).unwrap();
    }
    let max_v = if stones[top_i] > top_v {
        get_max(top_v, top_i - 1, stones, cache)
    } else {
        max(get_max(top_v, top_i - 1, stones, cache), stones[top_i] +
            get_max(top_v - stones[top_i], top_i - 1, stones, cache))
    };
    cache.insert(k, max_v);
    return max_v;
}

#[test]
fn test() {
    let stones = vec![13,24,22,15,32,14,23,21,30,17];
    println!("最后的重量为：{}", Solution::last_stone_weight_ii(stones));
}