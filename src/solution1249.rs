pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_remove_to_make_valid(s: String) -> String {
        use std::collections::HashSet;
        let mut current_set: HashSet<usize> = HashSet::new();
        let mut the_stack: Vec<usize> = Vec::new();

        for i in 0..s.len() {
            if s.chars().nth(i).unwrap() == '(' {
                the_stack.push(i);
            } 

            if s.chars().nth(i).unwrap() == ')' {
                if the_stack.len() == 0 {
                    current_set.insert(i);
                } else {
                    the_stack.pop();
                }
            }
        }

        while the_stack.len() != 0 {
            current_set.insert(the_stack.pop().unwrap());
        }

        s.chars()
            .enumerate()
            .filter(|(i, _x)| !current_set.contains(i))
            .map(|(_i, x)| x)
            .collect::<String>()
    }
}

/*

Java version

class Solution {
    public String minRemoveToMakeValid(String s) {
        Set<Integer> indexesToRemove = new HashSet<>();
        Stack<Integer> stack = new Stack<>();
        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) == '(') {
                stack.push(i);
            } if (s.charAt(i) == ')') {
                if (stack.isEmpty()) {
                    indexesToRemove.add(i);
                } else {
                    stack.pop();
                }
            }
        }
        // Put any indexes remaining on stack into the set.
        while (!stack.isEmpty()) indexesToRemove.add(stack.pop());
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < s.length(); i++) {
            if (!indexesToRemove.contains(i)) {
                sb.append(s.charAt(i));
            }
        }
        return sb.toString();
    }
}

*/