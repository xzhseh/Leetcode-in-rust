/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
struct MyQueue {
    stack_one: Vec<i32>,
    stack_two: Vec<i32>, 
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /* time: 100.00% space: 40.91% */
    #[allow(dead_code)]
    fn new() -> Self {
        MyQueue {
            stack_one: Vec::new(),
            stack_two: Vec::new(),
        }
    }
    
    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.stack_one.push(x);
    }
    
    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        if self.stack_two.len() != 0 {
            return self.stack_two.pop().unwrap();
        } else {
            while self.stack_one.len() != 0 {
                self.stack_two.push(self.stack_one.pop().unwrap());
            }
            return self.stack_two.pop().unwrap();
        }
    }
    
    #[allow(dead_code)]
    fn peek(&self) -> i32 {
        if self.stack_two.len() != 0 {
            self.stack_two[self.stack_two.len() - 1]
        } else {
            self.stack_one[0]
        }
    }
    
    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.stack_one.len() == 0 && self.stack_two.len() == 0
    }
}