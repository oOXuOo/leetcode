pub struct Solution{}

impl Solution {
    pub fn convert(s: String, n: i32) -> String {
        let num_rows = n as usize;
        if s.len() <= num_rows || num_rows == 1 { return s;}
        let v: Vec<char> = s.chars().collect();
        let num_loop = 2 * num_rows - 2;
        let mut result : Vec<char> = Vec::with_capacity(v.len());
        for row in 0..num_rows {
            if row >= v.len() { break;}
            let mut index = row;
            if row == 0 || row + 1 == num_rows {
                while index < v.len() {
                    result.push(v[index]);
                    index += num_loop;
                };
            } else {
                while index < v.len() {
                    result.push(v[index]);
                    if index+2*(num_rows-1-row) < v.len() {
                        result.push(v[index+2*(num_rows-1-row)]);
                    }
                    index += num_loop;
                };
            };
        }
        return result.into_iter().collect();
    }
}

#[test]
fn test() {
    let s = "PAYPALISHIRING".to_string();
    let x = Solution::convert(s, 3);
    println!("变形后:{}", x);
}