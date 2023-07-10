/**
 * [1206] Design Skiplist
 *
 * Design a Skiplist without using any built-in libraries.
 * A skiplist is a data structure that takes O(log(n)) time to add, erase and search. Comparing with treap and red-black tree which has the same function and performance, the code length of Skiplist can be comparatively short and the idea behind Skiplists is just simple linked lists.
 * For example, we have a Skiplist containing [30,40,50,60,70,90] and we want to add 80 and 45 into it. The Skiplist works this way:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/27/1506_skiplist.gif" style="width: 500px; height: 173px;" /><br />
 * <small>Artyom Kalinin [CC BY-SA 3.0], via <a href="https://commons.wikimedia.org/wiki/File:Skip_list_add_element-en.gif" target="_blank" title="Artyom Kalinin [CC BY-SA 3.0 (https://creativecommons.org/licenses/by-sa/3.0)], via Wikimedia Commons">Wikimedia Commons</a></small>
 * You can see there are many layers in the Skiplist. Each layer is a sorted linked list. With the help of the top layers, add, erase and search can be faster than O(n). It can be proven that the average time complexity for each operation is O(log(n)) and space complexity is O(n).
 * See more about Skiplist: <a href="https://en.wikipedia.org/wiki/Skip_list" target="_blank">https://en.wikipedia.org/wiki/Skip_list</a>
 * Implement the Skiplist class:
 *
 * 	Skiplist() Initializes the object of the skiplist.
 * 	bool search(int target) Returns true if the integer target exists in the Skiplist or false otherwise.
 * 	void add(int num) Inserts the value num into the SkipList.
 * 	bool erase(int num) Removes the value num from the Skiplist and returns true. If num does not exist in the Skiplist, do nothing and return false. If there exist multiple num values, removing any one of them is fine.
 *
 * Note that duplicates may exist in the Skiplist, your code needs to handle this situation.
 *  
 * Example 1:
 *
 * Input
 * ["Skiplist", "add", "add", "add", "search", "add", "search", "erase", "erase", "search"]
 * [[], [1], [2], [3], [0], [4], [1], [0], [1], [1]]
 * Output
 * [null, null, null, null, false, null, true, false, true, false]
 * Explanation
 * Skiplist skiplist = new Skiplist();
 * skiplist.add(1);
 * skiplist.add(2);
 * skiplist.add(3);
 * skiplist.search(0); // return False
 * skiplist.add(4);
 * skiplist.search(1); // return True
 * skiplist.erase(0);  // return False, 0 is not in skiplist.
 * skiplist.erase(1);  // return True
 * skiplist.search(1); // return False, 1 has already been erased.
 *  
 * Constraints:
 *
 * 	0 <= num, target <= 2 * 10^4
 * 	At most 5 * 10^4 calls will be made to search, add, and erase.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-skiplist/
// discuss: https://leetcode.com/problems/design-skiplist/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/design-skiplist/solutions/762661/rust-using-rc-and-only-right-and-down-pointers-idiomatic-code-with-pretty-printer/

type Link = std::rc::Rc<std::cell::RefCell<Node>>;

fn new_link(value: i32) -> Link {
    std::rc::Rc::new(std::cell::RefCell::new(Node::new(value)))
}

#[derive(Debug)]
struct Node {
    value: i32,
    right: Option<Link>,
    down: Option<Link>,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            value,
            right: None,
            down: None,
        }
    }
}

struct Skiplist {
    start: Link,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    fn new() -> Self {
        Skiplist {
            start: new_link(std::i32::MIN),
        }
    }

    fn find(&self, target: i32) -> Vec<Link> {
        let mut node_opt = Some(self.start.clone());
        let mut stack = vec![];

        while let Some(node) = node_opt {
            if node
                .borrow()
                .right
                .clone()
                .filter(|n| n.borrow().value <= target)
                .is_some()
            {
                node_opt = node.borrow().right.clone();
            } else {
                stack.push(node.clone());
                node_opt = node.borrow().down.clone();
            }
        }

        stack
    }

    fn search(&self, target: i32) -> bool {
        self.find(target)
            .last()
            .filter(|node| node.borrow().value == target)
            .is_some()
    }

    fn add(&mut self, num: i32) {
        let mut left_nodes = self.find(num);

        let mut node = new_link(num);

        let left_node = left_nodes.pop().unwrap();
        node.borrow_mut().right = left_node.borrow_mut().right.clone();
        left_node.borrow_mut().right = Some(node.clone());

        while rand::random() {
            let left_node = match left_nodes.pop() {
                Some(left_node) => left_node,
                None => {
                    let new_start = new_link(std::i32::MIN);
                    new_start.borrow_mut().down = Some(self.start.clone());
                    self.start = new_start;
                    self.start.clone()
                }
            };

            let new_node = new_link(num);
            new_node.borrow_mut().down = Some(node);
            new_node.borrow_mut().right = left_node.borrow_mut().right.clone();

            left_node.borrow_mut().right = Some(new_node.clone());

            node = new_node;
        }
    }

    fn erase(&self, num: i32) -> bool {
        let mut num_found = false;
        for node in self.find(num - 1) {
            let target_node_opt = node.borrow().right.clone();

            if let Some(target_node) = target_node_opt.filter(|n| n.borrow().value == num) {
                num_found = true;
                node.borrow_mut().right = target_node.borrow().right.clone();
                target_node.borrow_mut().right = None;
                target_node.borrow_mut().down = None;
            }
        }

        num_found
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1206_example_1() {
        let mut skiplist = Skiplist::new();
        skiplist.add(1);
        skiplist.add(2);
        skiplist.add(3);
        assert_eq!(skiplist.search(0), false); // return False
        skiplist.add(4);
        assert_eq!(skiplist.search(1), true); // return True
        assert_eq!(skiplist.erase(0), false); // return False, 0 is not in skiplist.
        assert_eq!(skiplist.erase(1), true); // return True
        assert_eq!(skiplist.search(1), false); // return False, 1 has already been erased.
    }
}
