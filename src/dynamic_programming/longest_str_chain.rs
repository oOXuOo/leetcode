use std::collections::HashMap;

///
/// 给出一个单词列表，其中每个单词都由小写英文字母组成。
/// 如果我们可以在word1的任何地方添加一个字母使其变成word2，
/// 那么我们认为word1是word2的前身。例如，"abc"是"abac"的前身。
///
/// 词链是单词[word_1, word_2, ..., word_k]组成的序列，k >= 1，
/// 其中word_1是word_2的前身，word_2是word_3的前身，依此类推。
///
/// 从给定单词列表 words 中选择单词组成词链，返回词链的最长可能长度。
/// 链接：https://leetcode-cn.com/problems/longest-string-chain
///
pub struct Solution {}

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut max = 1;
        let sub_map = &mut HashMap::new();
        for word in &words {
            let tmp = inner(word, &words, sub_map);
            if tmp > max { max = tmp;};
        };
        return max;
    }
}

fn inner(word: &String, words: &Vec<String>,
         sub_map: &mut HashMap<usize, i32>) -> i32 {
    let length = word.len();
    if length <= 1 { return 1; }
    let mut max = 1;
    for index in 0..length {
        let mut sub_tmp = word.clone();
        sub_tmp.replace_range(index..index + 1, "");
        let index_opt = get_index(&sub_tmp, words);
        if index_opt.is_none() {
            continue;
        };
        let sub = index_opt.unwrap();
        let opt = sub_map.get(&sub);
        let sub_max;
        if opt.is_some() {
            sub_max = *opt.unwrap();
        } else {
            sub_max = inner(&words[sub], words, sub_map);
        };
        sub_map.insert(sub, sub_max);
        if sub_max + 1 > max { max = sub_max + 1 };
    }
    return max;
}

fn get_index(name: &String, array: &Vec<String>) -> Option<usize> {
    for index in 0..array.len() {
        let s = &array[index];
        if s.eq(name) { return Some(index);}
    }
    return None;
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
#[test]
fn test() {
    let v = vec_of_strings!["a", "b", "ba", "bca", "bda", "bdca"];
    let max = Solution::longest_str_chain(v);
    println!("最长单词链长度为{}", max);
}