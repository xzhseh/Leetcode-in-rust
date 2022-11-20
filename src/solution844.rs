pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn backspace_compare(s: String, t: String) -> bool {
        let string_builder = |s: String| -> String {
            let mut return_string = String::new();
            
            for c in s.chars() {
                if c != '#' {
                    return_string.push(c);
                } else if !s.is_empty() {
                    return_string.pop();
                }
            }

            return_string
        };

        string_builder(s) == string_builder(t)
    }
}