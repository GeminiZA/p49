struct Solution;

use std::collections::HashMap;
impl Solution {

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut arr: Vec<char> = s.chars().collect();
            arr.sort();
            let key = arr.into_iter().collect();
            map.entry(key).and_modify(|value| value.push(s.clone())).or_insert(vec![s.clone()]);
        }
        let mut result: Vec<Vec<String>> = Vec::new();
        for (_, value) in map {
            result.push(value);
        }
        return result;
    }

}
fn main() {
    let input: Vec<&str> = vec!["a"];
    let strs: Vec<String> = input.iter().map(|s: &&str| s.to_string()).collect();
    let result = Solution::group_anagrams(strs);
    println!("{:?}", result);
}

