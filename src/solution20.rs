pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
		let mut stack: Vec<char> = Vec::new();
		let mut flag = true;
		let _tmp = s.chars().map(|x| {
			match x {
				')' => {
					match stack.pop() {
						Some(val) => {
							if val != '(' {
								flag = false;
							}
						},
						None => flag = false,
					}
				},
				']' => {
					match stack.pop() {
						Some(val) => {
							if val != '[' {
								flag = false;
							}
						},
						None => flag = false,
					}
				},
				'}' => {
					match stack.pop() {
						Some(val) => {
							if val != '{' {
								flag = false;
							}
						},
						None => flag = false,
					}
				},
				_ => stack.push(x),
			}
			x
		}).collect::<Vec<char>>();
        if stack.len() != 0 {
            flag = false;
        }
		flag
    }
}