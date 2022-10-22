pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut return_vec: Vec<Vec<String>> = Vec::new();
        let mut hashmap: HashMap<String, usize> = HashMap::new(); // The value here represent the index of the return_vec

        for elem in strs {
            let original = elem.clone();
            let mut _tmp = elem.chars().collect::<Vec<char>>();
            _tmp.sort();
            let elem = _tmp.iter().collect::<String>();
            if hashmap.contains_key(&elem) {
                return_vec[hashmap[&elem]].push(original);
            } else {
                return_vec.push(vec![original.clone()]);
                hashmap.insert(elem, return_vec.len() - 1);
            }
        }

        return_vec
    }
}