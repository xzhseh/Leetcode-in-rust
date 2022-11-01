/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
struct MinStack {
    cur_smallest: i32,
    the_stack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    #[allow(dead_code)]
    fn new() -> Self {
        MinStack { 
            cur_smallest: -1, 
            the_stack: Vec::new(), 
        }
    }
    
    #[allow(dead_code)]
    fn push(&mut self, val: i32) {
        if self.the_stack.len() == 0 {
            self.cur_smallest = val;
        }

        if val < self.cur_smallest {
            self.cur_smallest = val;
        }
        
        self.the_stack.push(val);
    }
    
    #[allow(dead_code)]
    fn pop(&mut self) {
        let val = self.the_stack.pop().unwrap();

        if self.the_stack.len() == 0 {
            return;
        }

        if val == self.cur_smallest {
            self.cur_smallest = self.the_stack[0];
            for i in 1..self.the_stack.len() {
                if self.the_stack[i] < self.cur_smallest {
                    self.cur_smallest = self.the_stack[i];
                }
            }
        }
    }
    
    #[allow(dead_code)]
    fn top(&self) -> i32 {
        self.the_stack[self.the_stack.len() - 1]
    }
    
    #[allow(dead_code)]
    fn get_min(&self) -> i32 {
        self.cur_smallest
    }
}