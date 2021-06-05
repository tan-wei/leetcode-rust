/**
 * [155] Min Stack
 *
 * Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
 * Implement the MinStack class:
 *
 * 	MinStack() initializes the stack object.
 * 	void push(val) pushes the element val onto the stack.
 * 	void pop() removes the element on the top of the stack.
 * 	int top() gets the top element of the stack.
 * 	int getMin() retrieves the minimum element in the stack.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MinStack","push","push","push","getMin","pop","top","getMin"]
 * [[],[-2],[0],[-3],[],[],[],[]]
 * Output
 * [null,null,null,null,-3,null,0,-2]
 * Explanation
 * MinStack minStack = new MinStack();
 * minStack.push(-2);
 * minStack.push(0);
 * minStack.push(-3);
 * minStack.getMin(); // return -3
 * minStack.pop();
 * minStack.top();    // return 0
 * minStack.getMin(); // return -2
 *
 *  
 * Constraints:
 *
 * 	-2^31 <= val <= 2^31 - 1
 * 	Methods pop, top and getMin operations will always be called on non-empty stacks.
 * 	At most 3 * 10^4 calls will be made to push, pop, top, and getMin.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/min-stack/
// discuss: https://leetcode.com/problems/min-stack/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct MinStack {
    v: Vec<i32>,
    m: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack { v: vec![], m: None }
    }

    fn push(&mut self, val: i32) {
        self.v.push(val);
        if let Some(m) = self.m {
            if val < m {
                self.m = Some(val);
            }
        }
    }

    fn pop(&mut self) {
        let x = self.v.pop().unwrap();
        if let Some(m) = self.m {
            if x == m {
                self.m = None;
            }
        }
    }

    fn top(&self) -> i32 {
        self.v[self.v.len() - 1]
    }

    fn get_min(&mut self) -> i32 {
        self.m.unwrap_or_else(|| {
            let m = self.v.iter().min().unwrap();
            let m = *m;
            self.m = Some(m);
            m
        })
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0155_example_1() {
        let mut min_satck = MinStack::new();
        min_satck.push(-2);
        min_satck.push(0);
        min_satck.push(-3);
        assert_eq!(min_satck.get_min(), -3);
        min_satck.pop();
        assert_eq!(min_satck.top(), 0);
        assert_eq!(min_satck.get_min(), -2);
    }
}
