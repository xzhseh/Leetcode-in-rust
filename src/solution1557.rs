pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut return_vec: Vec<i32> = Vec::new();
        let mut the_set: HashSet<i32> = HashSet::new();

        for edge in edges {
            the_set.insert(edge[1]);
        }

        for i in 0..n {
            if !the_set.contains(&i) {
                return_vec.push(i);
            }
        }

        return_vec
    }
}