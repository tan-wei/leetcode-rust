/**
 * [232] Implement Queue using Stacks
 *
 * Implement a first in first out (FIFO) queue using only two stacks. The implemented queue should support all the functions of a normal queue (push, peek, pop, and empty).
 * Implement the MyQueue class:
 *
 * 	void push(int x) Pushes element x to the back of the queue.
 * 	int pop() Removes the element from the front of the queue and returns it.
 * 	int peek() Returns the element at the front of the queue.
 * 	boolean empty() Returns true if the queue is empty, false otherwise.
 *
 * Notes:
 *
 * 	You must use only standard operations of a stack, which means only push to top, peek/pop from top, size, and is empty operations are valid.
 * 	Depending on your language, the stack may not be supported natively. You may simulate a stack using a list or deque (double-ended queue) as long as you use only a stack's standard operations.
 *
 * Follow-up: Can you implement the queue such that each operation is <a href="https://en.wikipedia.org/wiki/Amortized_analysis" target="_blank">amortized</a> O(1) time complexity? In other words, performing n operations will take overall O(n) time even if one of those operations may take longer.
 *  
 * Example 1:
 *
 * Input
 * ["MyQueue", "push", "push", "peek", "pop", "empty"]
 * [[], [1], [2], [], [], []]
 * Output
 * [null, null, null, 1, 1, false]
 * Explanation
 * MyQueue myQueue = new MyQueue();
 * myQueue.push(1); // queue is: [1]
 * myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
 * myQueue.peek(); // return 1
 * myQueue.pop(); // return 1, queue is [2]
 * myQueue.empty(); // return false
 *
 *  
 * Constraints:
 *
 * 	1 <= x <= 9
 * 	At most 100 calls will be made to push, pop, peek, and empty.
 * 	All the calls to pop and peek are valid.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/implement-queue-using-stacks/
// discuss: https://leetcode.com/problems/implement-queue-using-stacks/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }
    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }
    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while !self.stack_in.is_empty() {
                self.stack_out.push(self.stack_in.pop().unwrap());
            }
        }
        self.stack_out.pop().unwrap()
    }
    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while !self.stack_in.is_empty() {
                self.stack_out.push(self.stack_in.pop().unwrap());
            }
        }
        *self.stack_out.last().unwrap()
    }
    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0232_example_1() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);
        assert_eq!(obj.empty(), false);
    }
}
