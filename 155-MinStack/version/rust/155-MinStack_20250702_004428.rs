// Last updated: 7/2/2025, 12:44:28 AM
use std::collections::VecDeque;

struct MinStack 
{
    stack: VecDeque<i32>,
    minStack: VecDeque<(i32, i32)>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self 
    {
        MinStack {
            stack: VecDeque::new(),
            minStack: VecDeque::new()
        }
    }
    
    fn push(&mut self, val: i32) 
    {
        self.stack.push_back(val);

        if self.minStack.is_empty() || self.minStack.back().unwrap().0 > val
        {
            self.minStack.push_back((val, 1));
        }

        else if val == self.minStack.back().unwrap().0
        {
            if let Some(top) = self.minStack.back_mut() { top.1 += 1; }
        }
    }
    
    fn pop(&mut self) 
    {
        let popTop = self.stack.pop_back();

        if let Some(val) = popTop
        {
            if let Some(minTopTuple) = self.minStack.back_mut()
            {
                if val == minTopTuple.0
                {
                    minTopTuple.1 -= 1;
                    if minTopTuple.1 == 0 { self.minStack.pop_back(); }
                }
            }
        }    
    }
    
    fn top(&self) -> i32 { self.stack.back().unwrap().clone() }
    
    fn get_min(&self) -> i32 { self.minStack.back().unwrap().0 }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
