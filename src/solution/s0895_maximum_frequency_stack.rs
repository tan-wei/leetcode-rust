/**
 * [0895] Maximum Frequency Stack
 *
 * Design a stack-like data structure to push elements to the stack and pop the most frequent element from the stack.
 * Implement the FreqStack class:
 *
 * 	FreqStack() constructs an empty frequency stack.
 * 	void push(int val) pushes an integer val onto the top of the stack.
 * 	int pop() removes and returns the most frequent element in the stack.
 *
 * 		If there is a tie for the most frequent element, the element closest to the stack's top is removed and returned.
 *
 *
 *
 *  
 * Example 1:
 *
 * Input
 * ["FreqStack", "push", "push", "push", "push", "push", "push", "pop", "pop", "pop", "pop"]
 * [[], [5], [7], [5], [7], [4], [5], [], [], [], []]
 * Output
 * [null, null, null, null, null, null, null, 5, 7, 5, 4]
 * Explanation
 * FreqStack freqStack = new FreqStack();
 * freqStack.push(5); // The stack is [5]
 * freqStack.push(7); // The stack is [5,7]
 * freqStack.push(5); // The stack is [5,7,5]
 * freqStack.push(7); // The stack is [5,7,5,7]
 * freqStack.push(4); // The stack is [5,7,5,7,4]
 * freqStack.push(5); // The stack is [5,7,5,7,4,5]
 * freqStack.pop();   // return 5, as 5 is the most frequent. The stack becomes [5,7,5,7,4].
 * freqStack.pop();   // return 7, as 5 and 7 is the most frequent, but 7 is closest to the top. The stack becomes [5,7,5,4].
 * freqStack.pop();   // return 5, as 5 is the most frequent. The stack becomes [5,7,4].
 * freqStack.pop();   // return 4, as 4, 5 and 7 is the most frequent, but 4 is closest to the top. The stack becomes [5,7].
 *
 *  
 * Constraints:
 *
 * 	0 <= val <= 10^9
 * 	At most 2 * 10^4 calls will be made to push and pop.
 * 	It is guaranteed that there will be at least one element in the stack before calling pop.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-frequency-stack/
// discuss: https://leetcode.com/problems/maximum-frequency-stack/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Default)]
struct FreqStack {
    freq: std::collections::HashMap<i32, usize>,
    group: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        let entry = self.freq.entry(val).or_insert(0);
        if *entry >= self.group.len() {
            self.group.push(Vec::new());
        }
        self.group[*entry].push(val);
        *entry += 1;
    }

    fn pop(&mut self) -> i32 {
        if let Some(maxfreq) = self.group.last_mut() {
            if let Some(ret) = maxfreq.pop() {
                *self.freq.entry(ret).or_default() -= 1;
                if maxfreq.is_empty() {
                    self.group.pop();
                }
                return ret;
            }
        }
        unreachable!()
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0895_example_1() {
        let mut freq_stack = FreqStack::new();
        freq_stack.push(5); // The stack is [5]
        freq_stack.push(7); // The stack is [5,7]
        freq_stack.push(5); // The stack is [5,7,5]
        freq_stack.push(7); // The stack is [5,7,5,7]
        freq_stack.push(4); // The stack is [5,7,5,7,4]
        freq_stack.push(5); // The stack is [5,7,5,7,4,5]
        assert_eq!(freq_stack.pop(), 5); // return 5, as 5 is the most frequent. The stack becomes [5,7,5,7,4].
        assert_eq!(freq_stack.pop(), 7); // return 7, as 5 and 7 is the most frequent, but 7 is closest to the top. The stack becomes [5,7,5,4].
        assert_eq!(freq_stack.pop(), 5); // return 5, as 5 is the most frequent. The stack becomes [5,7,4].
        assert_eq!(freq_stack.pop(), 4); // return 4, as 4, 5 and 7 is the most frequent, but 4 is closest to the top. The stack becomes [5,7].
    }
}
