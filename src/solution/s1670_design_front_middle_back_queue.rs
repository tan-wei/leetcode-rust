/**
 * [1670] Design Front Middle Back Queue
 *
 * Design a queue that supports push and pop operations in the front, middle, and back.
 * Implement the FrontMiddleBack class:
 *
 * 	FrontMiddleBack() Initializes the queue.
 * 	void pushFront(int val) Adds val to the front of the queue.
 * 	void pushMiddle(int val) Adds val to the middle of the queue.
 * 	void pushBack(int val) Adds val to the back of the queue.
 * 	int popFront() Removes the front element of the queue and returns it. If the queue is empty, return -1.
 * 	int popMiddle() Removes the middle element of the queue and returns it. If the queue is empty, return -1.
 * 	int popBack() Removes the back element of the queue and returns it. If the queue is empty, return -1.
 *
 * Notice that when there are two middle position choices, the operation is performed on the frontmost middle position choice. For example:
 *
 * 	Pushing 6 into the middle of [1, 2, 3, 4, 5] results in [1, 2, <u>6</u>, 3, 4, 5].
 * 	Popping the middle from [1, 2, <u>3</u>, 4, 5, 6] returns 3 and results in [1, 2, 4, 5, 6].
 *
 *  
 * Example 1:
 *
 * Input:
 * ["FrontMiddleBackQueue", "pushFront", "pushBack", "pushMiddle", "pushMiddle", "popFront", "popMiddle", "popMiddle", "popBack", "popFront"]
 * [[], [1], [2], [3], [4], [], [], [], [], []]
 * Output:
 * [null, null, null, null, null, 1, 3, 4, 2, -1]
 * Explanation:
 * FrontMiddleBackQueue q = new FrontMiddleBackQueue();
 * q.pushFront(1);   // [<u>1</u>]
 * q.pushBack(2);    // [1, <u>2</u>]
 * q.pushMiddle(3);  // [1, <u>3</u>, 2]
 * q.pushMiddle(4);  // [1, <u>4</u>, 3, 2]
 * q.popFront();     // return 1 -> [4, 3, 2]
 * q.popMiddle();    // return 3 -> [4, 2]
 * q.popMiddle();    // return 4 -> [2]
 * q.popBack();      // return 2 -> []
 * q.popFront();     // return -1 -> [] (The queue is empty)
 *
 *  
 * Constraints:
 *
 * 	1 <= val <= 10^9
 * 	At most 1000 calls will be made to pushFront, pushMiddle, pushBack, popFront, popMiddle, and popBack.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-front-middle-back-queue/
// discuss: https://leetcode.com/problems/design-front-middle-back-queue/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct FrontMiddleBackQueue {
    vec: std::collections::VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            vec: std::collections::VecDeque::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        self.vec.push_back(val);
    }

    fn push_middle(&mut self, val: i32) {
        let size = self.vec.len();
        self.vec.insert(size / 2, val)
    }

    fn push_back(&mut self, val: i32) {
        self.vec.push_back(val);
    }

    fn pop_front(&mut self) -> i32 {
        self.vec.pop_front().unwrap_or(-1)
    }

    fn pop_middle(&mut self) -> i32 {
        let mut size = self.vec.len();
        if size % 2 == 1 {
            size /= 2;
        } else {
            size = size / 2 - 1;
        }
        self.vec.remove(size).unwrap_or(-1)
    }

    fn pop_back(&mut self) -> i32 {
        self.vec.pop_back().unwrap_or(-1)
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1670_example_1() {
        let mut q = FrontMiddleBackQueue::new();
        q.push_front(1); // [<u>1</u>]
        q.push_back(2); // [1, <u>2</u>]
        q.push_middle(3); // [1, <u>3</u>, 2]
        q.push_middle(4); // [1, <u>4</u>, 3, 2]
        assert_eq!(q.pop_front(), 1); // return 1 -> [4, 3, 2]
        assert_eq!(q.pop_middle(), 3); // return 3 -> [4, 2]
        assert_eq!(q.pop_middle(), 4); // return 4 -> [2]
        assert_eq!(q.pop_back(), 2); // return 2 -> []
        assert_eq!(q.pop_front(), -1); // return -1 -> [] (The queue is empty)
    }
}
