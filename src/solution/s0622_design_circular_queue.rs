/**
 * [0622] Design Circular Queue
 *
 * Design your implementation of the circular queue. The circular queue is a linear data structure in which the operations are performed based on FIFO (First In First Out) principle and the last position is connected back to the first position to make a circle. It is also called "Ring Buffer".
 * One of the benefits of the circular queue is that we can make use of the spaces in front of the queue. In a normal queue, once the queue becomes full, we cannot insert the next element even if there is a space in front of the queue. But using the circular queue, we can use the space to store new values.
 * Implementation the MyCircularQueue class:
 *
 * 	MyCircularQueue(k) Initializes the object with the size of the queue to be k.
 * 	int Front() Gets the front item from the queue. If the queue is empty, return -1.
 * 	int Rear() Gets the last item from the queue. If the queue is empty, return -1.
 * 	boolean enQueue(int value) Inserts an element into the circular queue. Return true if the operation is successful.
 * 	boolean deQueue() Deletes an element from the circular queue. Return true if the operation is successful.
 * 	boolean isEmpty() Checks whether the circular queue is empty or not.
 * 	boolean isFull() Checks whether the circular queue is full or not.
 *
 * You must solve the problem without using the built-in queue data structure in your programming language.
 *  
 * Example 1:
 *
 * Input
 * ["MyCircularQueue", "enQueue", "enQueue", "enQueue", "enQueue", "Rear", "isFull", "deQueue", "enQueue", "Rear"]
 * [[3], [1], [2], [3], [4], [], [], [], [4], []]
 * Output
 * [null, true, true, true, false, 3, true, true, true, 4]
 * Explanation
 * MyCircularQueue myCircularQueue = new MyCircularQueue(3);
 * myCircularQueue.enQueue(1); // return True
 * myCircularQueue.enQueue(2); // return True
 * myCircularQueue.enQueue(3); // return True
 * myCircularQueue.enQueue(4); // return False
 * myCircularQueue.Rear();     // return 3
 * myCircularQueue.isFull();   // return True
 * myCircularQueue.deQueue();  // return True
 * myCircularQueue.enQueue(4); // return True
 * myCircularQueue.Rear();     // return 4
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= 1000
 * 	0 <= value <= 1000
 * 	At most 3000 calls will be made to enQueue, deQueue, Front, Rear, isEmpty, and isFull.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-circular-queue/
// discuss: https://leetcode.com/problems/design-circular-queue/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct MyCircularQueue {
    v: Vec<i32>,
    front: usize,
    rear: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            v: vec![0; k as usize + 1],
            front: 0,
            rear: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.v[self.rear] = value;
        self.rear = (self.rear + 1) % self.v.len();
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.v.len();
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.v[self.front]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.v[(self.rear + self.v.len() - 1) % self.v.len()]
        }
    }

    fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    fn is_full(&self) -> bool {
        (self.rear + 1) % self.v.len() == self.front
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0622_example_1() {
        let mut obj = MyCircularQueue::new(3);
        assert_eq!(obj.en_queue(1), true);
        assert_eq!(obj.en_queue(2), true);
        assert_eq!(obj.en_queue(3), true);
        assert_eq!(obj.en_queue(4), false);
        assert_eq!(obj.rear(), 3);
        assert_eq!(obj.is_full(), true);
        assert_eq!(obj.de_queue(), true);
        assert_eq!(obj.en_queue(4), true);
        assert_eq!(obj.rear(), 4);
    }
}
