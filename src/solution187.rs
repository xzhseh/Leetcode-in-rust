pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashMap;
        let mut hashmap: HashMap<String, i32> = HashMap::new();
        let mut return_vec: Vec<String> = Vec::new();
        if s.len() <= 10 {
            return_vec
        } else {
            for i in 0..=(s.len() - 10) {
                if hashmap.contains_key(&s[i..(i + 10)].to_string()) {
                    hashmap.entry(s[i..(i + 10)].to_string()).and_modify(|x| *x += 1);
                    if hashmap[&s[i..(i + 10)].to_string()] == 2 {
                        return_vec.push(s[i..(i + 10)].to_string());
                    }
                } else {
                    hashmap.insert(s[i..(i + 10)].to_string(), 1);
                }
            }

            return_vec
        }  
    }
}