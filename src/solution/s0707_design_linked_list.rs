/**
 * [0707] Design Linked List
 *
 * Design your implementation of the linked list. You can choose to use a singly or doubly linked list.<br />
 * A node in a singly linked list should have two attributes: val and next. val is the value of the current node, and next is a pointer/reference to the next node.<br />
 * If you want to use the doubly linked list, you will need one more attribute prev to indicate the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.
 * Implement the MyLinkedList class:
 *
 * 	MyLinkedList() Initializes the MyLinkedList object.
 * 	int get(int index) Get the value of the index^th node in the linked list. If the index is invalid, return -1.
 * 	void addAtHead(int val) Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
 * 	void addAtTail(int val) Append a node of value val as the last element of the linked list.
 * 	void addAtIndex(int index, int val) Add a node of value val before the index^th node in the linked list. If index equals the length of the linked list, the node will be appended to the end of the linked list. If index is greater than the length, the node will not be inserted.
 * 	void deleteAtIndex(int index) Delete the index^th node in the linked list, if the index is valid.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get", "deleteAtIndex", "get"]
 * [[], [1], [3], [1, 2], [1], [1], [1]]
 * Output
 * [null, null, null, null, 2, null, 3]
 * Explanation
 * MyLinkedList myLinkedList = new MyLinkedList();
 * myLinkedList.addAtHead(1);
 * myLinkedList.addAtTail(3);
 * myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
 * myLinkedList.get(1);              // return 2
 * myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
 * myLinkedList.get(1);              // return 3
 *
 *  
 * Constraints:
 *
 * 	0 <= index, val <= 1000
 * 	Please do not use the built-in LinkedList library.
 * 	At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and deleteAtIndex.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-linked-list/
// discuss: https://leetcode.com/problems/design-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/design-linked-list/discuss/1003187/Rust-solution

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[derive(Default)]
struct MyLinkedList {
    head: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        Default::default()
    }

    fn get(&self, index: i32) -> i32 {
        let mut cur = match self.head {
            Some(ref a) => a,
            None => return -1,
        };
        let mut idx_cur = 0;
        while idx_cur < index {
            if let Some(ref next) = cur.next {
                cur = next;
                idx_cur += 1;
            } else {
                return -1;
            };
        }
        cur.val
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }))
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut cur = match self.head {
            Some(ref mut a) => a,
            None => {
                self.head = Some(Box::new(Node { val, next: None }));
                return;
            }
        };
        while let Some(ref mut next) = cur.next {
            cur = next;
        }
        cur.next = Some(Box::new(Node { val, next: None }));
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            idx += 1;
        }
        cur.next = Some(Box::new(Node {
            val,
            next: cur.next.take(),
        }));
        self.head = dummy_head.next;
    }

    fn delete_at_index(&mut self, index: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            idx += 1;
        }
        cur.next = cur.next.take().and_then(|a| a.next);
        self.head = dummy_head.next;
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0707_example_1() {
        let mut my_linked_list = MyLinkedList::new();
        my_linked_list.add_at_head(1);
        my_linked_list.add_at_tail(3);
        my_linked_list.add_at_index(1, 2); // linked list becomes 1->2->3
        assert_eq!(my_linked_list.get(1), 2); // return 2
        my_linked_list.delete_at_index(1); // now the linked list is 1->3
        assert_eq!(my_linked_list.get(1), 3); // return 3
    }
}
