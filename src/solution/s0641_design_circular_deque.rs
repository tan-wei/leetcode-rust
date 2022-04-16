/**
 * [0641] Design Circular Deque
 *
 * Design your implementation of the circular double-ended queue (deque).
 * Implement the MyCircularDeque class:
 *
 * 	MyCircularDeque(int k) Initializes the deque with a maximum size of k.
 * 	boolean insertFront() Adds an item at the front of Deque. Returns true if the operation is successful, or false otherwise.
 * 	boolean insertLast() Adds an item at the rear of Deque. Returns true if the operation is successful, or false otherwise.
 * 	boolean deleteFront() Deletes an item from the front of Deque. Returns true if the operation is successful, or false otherwise.
 * 	boolean deleteLast() Deletes an item from the rear of Deque. Returns true if the operation is successful, or false otherwise.
 * 	int getFront() Returns the front item from the Deque. Returns -1 if the deque is empty.
 * 	int getRear() Returns the last item from Deque. Returns -1 if the deque is empty.
 * 	boolean isEmpty() Returns true if the deque is empty, or false otherwise.
 * 	boolean isFull() Returns true if the deque is full, or false otherwise.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MyCircularDeque", "insertLast", "insertLast", "insertFront", "insertFront", "getRear", "isFull", "deleteLast", "insertFront", "getFront"]
 * [[3], [1], [2], [3], [4], [], [], [], [4], []]
 * Output
 * [null, true, true, true, false, 2, true, true, true, 4]
 * Explanation
 * MyCircularDeque myCircularDeque = new MyCircularDeque(3);
 * myCircularDeque.insertLast(1);  // return True
 * myCircularDeque.insertLast(2);  // return True
 * myCircularDeque.insertFront(3); // return True
 * myCircularDeque.insertFront(4); // return False, the queue is full.
 * myCircularDeque.getRear();      // return 2
 * myCircularDeque.isFull();       // return True
 * myCircularDeque.deleteLast();   // return True
 * myCircularDeque.insertFront(4); // return True
 * myCircularDeque.getFront();     // return 4
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= 1000
 * 	0 <= value <= 1000
 * 	At most 2000 calls will be made to insertFront, insertLast, deleteFront, deleteLast, getFront, getRear, isEmpty, isFull.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-circular-deque/
// discuss: https://leetcode.com/problems/design-circular-deque/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct MyCircularDeque {
    k: usize,
    start: usize,
    end: usize,
    data: Vec<i32>,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            k: k as usize,
            start: 0,
            end: 0,
            data: vec![0; k as usize],
            count: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.count == self.k {
            false
        } else {
            self.count += 1;
            self.start = (self.start + self.k - 1) % self.k;
            self.data[self.start] = value;
            true
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.count == self.k {
            false
        } else {
            self.count += 1;
            self.data[self.end] = value;
            self.end = (self.end + 1) % self.k;
            true
        }
    }

    fn delete_front(&mut self) -> bool {
        if self.count == 0 {
            false
        } else {
            self.count -= 1;
            self.start = (self.start + 1) % self.k;
            true
        }
    }

    fn delete_last(&mut self) -> bool {
        if self.count == 0 {
            false
        } else {
            self.count -= 1;
            self.end = (self.end + self.k - 1) % self.k;
            true
        }
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.start]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.end + self.k - 1) % self.k]
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count == self.k
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0641_example_1() {
        let mut circular_deque = MyCircularDeque::new(3);
        assert_eq!(circular_deque.insert_last(1), true);
        assert_eq!(circular_deque.insert_last(2), true);
        assert_eq!(circular_deque.insert_front(3), true);
        assert_eq!(circular_deque.insert_front(4), false);
        assert_eq!(circular_deque.get_rear(), 2);
        assert_eq!(circular_deque.is_full(), true);
        assert_eq!(circular_deque.delete_last(), true);
        assert_eq!(circular_deque.insert_front(4), true);
        assert_eq!(circular_deque.get_front(), 4);
    }
}
