/**
 * [1172] Dinner Plate Stacks
 *
 * You have an infinite number of stacks arranged in a row and numbered (left to right) from 0, each of the stacks has the same maximum capacity.
 * Implement the DinnerPlates class:
 *
 * 	DinnerPlates(int capacity) Initializes the object with the maximum capacity of the stacks capacity.
 * 	void push(int val) Pushes the given integer val into the leftmost stack with a size less than capacity.
 * 	int pop() Returns the value at the top of the rightmost non-empty stack and removes it from that stack, and returns -1 if all the stacks are empty.
 * 	int popAtStack(int index) Returns the value at the top of the stack with the given index index and removes it from that stack or returns -1 if the stack with that given index is empty.
 *
 *  
 * Example 1:
 *
 * Input
 * ["DinnerPlates", "push", "push", "push", "push", "push", "popAtStack", "push", "push", "popAtStack", "popAtStack", "pop", "pop", "pop", "pop", "pop"]
 * [[2], [1], [2], [3], [4], [5], [0], [20], [21], [0], [2], [], [], [], [], []]
 * Output
 * [null, null, null, null, null, null, 2, null, null, 20, 21, 5, 4, 3, 1, -1]
 * Explanation:
 * DinnerPlates D = DinnerPlates(2);  // Initialize with capacity = 2
 * D.push(1);
 * D.push(2);
 * D.push(3);
 * D.push(4);
 * D.push(5);         // The stacks are now:  2  4
 *                                            1  3  5
 *                                            ﹈ ﹈ ﹈
 * D.popAtStack(0);   // Returns 2.  The stacks are now:     4
 *                                                        1  3  5
 *                                                        ﹈ ﹈ ﹈
 * D.push(20);        // The stacks are now: 20  4
 *                                            1  3  5
 *                                            ﹈ ﹈ ﹈
 * D.push(21);        // The stacks are now: 20  4 21
 *                                            1  3  5
 *                                            ﹈ ﹈ ﹈
 * D.popAtStack(0);   // Returns 20.  The stacks are now:     4 21
 *                                                         1  3  5
 *                                                         ﹈ ﹈ ﹈
 * D.popAtStack(2);   // Returns 21.  The stacks are now:     4
 *                                                         1  3  5
 *                                                         ﹈ ﹈ ﹈
 * D.pop()            // Returns 5.  The stacks are now:      4
 *                                                         1  3
 *                                                         ﹈ ﹈  
 * D.pop()            // Returns 4.  The stacks are now:   1  3
 *                                                         ﹈ ﹈   
 * D.pop()            // Returns 3.  The stacks are now:   1
 *                                                         ﹈   
 * D.pop()            // Returns 1.  There are no stacks.
 * D.pop()            // Returns -1.  There are still no stacks.
 *
 *  
 * Constraints:
 *
 * 	1 <= capacity <= 2 * 10^4
 * 	1 <= val <= 2 * 10^4
 * 	0 <= index <= 10^5
 * 	At most 2 * 10^5 calls will be made to push, pop, and popAtStack.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/dinner-plate-stacks/
// discuss: https://leetcode.com/problems/dinner-plate-stacks/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/dinner-plate-stacks/solutions/930743/2-rust-solutions-faster-than-100-using-binaryheap/

use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
struct DinnerPlates {
    stacks: Vec<Vec<i32>>,
    not_full_queue: std::collections::BinaryHeap<std::cmp::Reverse<usize>>,
    max_capacity: usize,
    last_poppable: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            stacks: Vec::new(),
            not_full_queue: std::collections::BinaryHeap::new(),
            max_capacity: capacity.try_into().unwrap(),
            last_poppable: 0,
        }
    }

    fn push(&mut self, val: i32) {
        if let Some(&std::cmp::Reverse(left_most_non_empty)) = self.not_full_queue.peek() {
            self.stacks[left_most_non_empty].push(val);
            if self.stacks[left_most_non_empty].len() >= self.max_capacity {
                self.not_full_queue.pop();
            }
            if self.last_poppable < left_most_non_empty {
                self.last_poppable = left_most_non_empty;
            }
        } else {
            self.stacks.push(Vec::with_capacity(self.max_capacity));
            self.stacks.last_mut().unwrap().push(val);
            self.last_poppable = self.stacks.len() - 1;
            if self.max_capacity > 1 {
                self.not_full_queue
                    .push(std::cmp::Reverse(self.stacks.len() - 1));
            }
        }
    }

    fn pop(&mut self) -> i32 {
        self.pop_at_idx(self.last_poppable)
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        self.pop_at_idx(index.try_into().unwrap())
    }

    fn pop_at_idx(&mut self, index: usize) -> i32 {
        if index >= self.stacks.len() {
            return -1;
        }
        let was_full = self.stacks[index].len() == self.max_capacity;
        let result = self.stacks[index].pop().unwrap_or(-1);
        if was_full {
            self.not_full_queue.push(std::cmp::Reverse(index));
        }
        if index == self.last_poppable && self.stacks[index].is_empty() {
            while self.stacks[self.last_poppable].is_empty() && self.last_poppable > 0 {
                self.last_poppable -= 1;
            }
        }
        result
    }
}

/**
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1172_example_1() {
        let mut d = DinnerPlates::new(2); // Initialize with capacity = 2
        d.push(1);
        d.push(2);
        d.push(3);
        d.push(4);
        d.push(5);

        assert_eq!(d.pop_at_stack(0), 2);
        d.push(20);
        d.push(21);

        assert_eq!(d.pop_at_stack(0), 20);
        assert_eq!(d.pop_at_stack(2), 21);
        assert_eq!(d.pop(), 5);
        assert_eq!(d.pop(), 4);
        assert_eq!(d.pop(), 3);
        assert_eq!(d.pop(), 1);
        assert_eq!(d.pop(), -1);
    }
}
