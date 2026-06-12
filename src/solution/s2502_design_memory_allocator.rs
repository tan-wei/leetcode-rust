/**
 * [2502] Design Memory Allocator
 *
 * You are given an integer n representing the size of a 0-indexed memory array. All memory units are initially free.
 * You have a memory allocator with the following functionalities:
 * <ol>
 * 	Allocate a block of size consecutive free memory units and assign it the id mID.
 * 	Free all memory units with the given id mID.
 * </ol>
 * Note that:
 *
 * 	Multiple blocks can be allocated to the same mID.
 * 	You should free all the memory units with mID, even if they were allocated in different blocks.
 *
 * Implement the Allocator class:
 *
 * 	Allocator(int n) Initializes an Allocator object with a memory array of size n.
 * 	int allocate(int size, int mID) Find the leftmost block of size consecutive free memory units and allocate it with the id mID. Return the block's first index. If such a block does not exist, return -1.
 * 	int freeMemory(int mID) Free all memory units with the id mID. Return the number of memory units you have freed.
 *
 *  
 * Example 1:
 *
 * Input
 * ["Allocator", "allocate", "allocate", "allocate", "freeMemory", "allocate", "allocate", "allocate", "freeMemory", "allocate", "freeMemory"]
 * [[10], [1, 1], [1, 2], [1, 3], [2], [3, 4], [1, 1], [1, 1], [1], [10, 2], [7]]
 * Output
 * [null, 0, 1, 2, 1, 3, 1, 6, 3, -1, 0]
 * Explanation
 * Allocator loc = new Allocator(10); // Initialize a memory array of size 10. All memory units are initially free.
 * loc.allocate(1, 1); // The leftmost block's first index is 0. The memory array becomes [1,_,_,_,_,_,_,_,_,_]. We return 0.
 * loc.allocate(1, 2); // The leftmost block's first index is 1. The memory array becomes [1,2,_,_,_,_,_,_,_,_]. We return 1.
 * loc.allocate(1, 3); // The leftmost block's first index is 2. The memory array becomes [1,2,3,_,_,_,_,_,_,_]. We return 2.
 * loc.freeMemory(2); // Free all memory units with mID 2. The memory array becomes [1,_, 3,_,_,_,_,_,_,_]. We return 1 since there is only 1 unit with mID 2.
 * loc.allocate(3, 4); // The leftmost block's first index is 3. The memory array becomes [1,_,3,4,4,4,_,_,_,_]. We return 3.
 * loc.allocate(1, 1); // The leftmost block's first index is 1. The memory array becomes [1,1,3,4,4,4,_,_,_,_]. We return 1.
 * loc.allocate(1, 1); // The leftmost block's first index is 6. The memory array becomes [1,1,3,4,4,4,1,_,_,_]. We return 6.
 * loc.freeMemory(1); // Free all memory units with mID 1. The memory array becomes [_,_,3,4,4,4,_,_,_,_]. We return 3 since there are 3 units with mID 1.
 * loc.allocate(10, 2); // We can not find any free block with 10 consecutive free memory units, so we return -1.
 * loc.freeMemory(7); // Free all memory units with mID 7. The memory array remains the same since there is no memory unit with mID 7. We return 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= n, size, mID <= 1000
 * 	At most 1000 calls will be made to allocate and freeMemory.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-memory-allocator/
// discuss: https://leetcode.com/problems/design-memory-allocator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/design-memory-allocator/solutions/3388396/btreemap-hashmap-by-ymlchris-yx8o/

struct Allocator {
    empties: std::collections::BTreeMap<i32, i32>,
    ids: std::collections::HashMap<i32, std::collections::BTreeMap<i32, i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Allocator {
    fn new(n: i32) -> Self {
        let mut empties = std::collections::BTreeMap::new();
        empties.insert(0, n - 1);
        return Allocator {
            empties,
            ids: std::collections::HashMap::new(),
        };
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut result_x = -1;
        let mut result_y = -1;
        for (k, v) in self.empties.iter() {
            if size <= *v - *k + 1 {
                result_x = *k;
                result_y = *v;
                break;
            }
        }
        if result_x != -1 {
            self.empties.remove(&result_x);
            if result_y - result_x + 1 > size {
                self.empties.insert(result_x + size, result_y);
            }
            self.ids
                .entry(m_id)
                .or_insert(std::collections::BTreeMap::new());
            Self::merge(
                self.ids.get_mut(&m_id).unwrap(),
                result_x,
                size + result_x - 1,
            );
        }
        result_x
    }

    fn free_memory(&mut self, m_id: i32) -> i32 {
        let mut result = 0;
        let empties = &mut self.empties;
        self.ids.entry(m_id).and_modify(|o| {
            o.retain(|result_x, result_y| {
                result += *result_y + 1 - *result_x;
                Self::merge(empties, *result_x, *result_y);
                false
            });
        });
        result
    }

    fn merge(sorted_list: &mut std::collections::BTreeMap<i32, i32>, result_x: i32, result_y: i32) {
        let mut back_x = -1;
        let mut final_x = sorted_list
            .range(..result_x)
            .next_back()
            .map_or(result_x, |(k, v)| {
                if result_x == 1 + *v {
                    back_x = *k;
                    return *k;
                }
                result_x
            });
        let mut forward_x = -1;
        let mut final_y = sorted_list
            .range((result_y + 1)..)
            .next()
            .map_or(result_y, |(k, v)| {
                if result_y == *k - 1 {
                    forward_x = *k;
                    return *v;
                }
                result_y
            });
        if back_x != -1 {
            sorted_list.remove(&back_x);
        }
        if forward_x != -1 {
            sorted_list.remove(&forward_x);
        }
        sorted_list.insert(final_x, final_y);
    }
}

/**
 * Your Allocator object will be instantiated and called as such:
 * let obj = Allocator::new(n);
 * let ret_1: i32 = obj.allocate(size, mID);
 * let ret_2: i32 = obj.free_memory(mID);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2502_example_1() {
        let mut loc = Allocator::new(10); // Initialize a memory array of size 10. All memory units are initially free.
        assert_eq!(loc.allocate(1, 1), 0); // The leftmost block's first index is 0. The memory array becomes [1,_,_,_,_,_,_,_,_,_]. We return 0.
        assert_eq!(loc.allocate(1, 2), 1); // The leftmost block's first index is 1. The memory array becomes [1,2,_,_,_,_,_,_,_,_]. We return 1.
        assert_eq!(loc.allocate(1, 3), 2); // The leftmost block's first index is 2. The memory array becomes [1,2,3,_,_,_,_,_,_,_]. We return 2.
        assert_eq!(loc.free_memory(2), 1); // Free all memory units with mID 2. The memory array becomes [1,_, 3,_,_,_,_,_,_,_]. We return 1 since there is only 1 unit with mID 2.
        assert_eq!(loc.allocate(3, 4), 3); // The leftmost block's first index is 3. The memory array becomes [1,_,3,4,4,4,_,_,_,_]. We return 3.
        assert_eq!(loc.allocate(1, 1), 1); // The leftmost block's first index is 1. The memory array becomes [1,1,3,4,4,4,_,_,_,_]. We return 1.
        assert_eq!(loc.allocate(1, 1), 6); // The leftmost block's first index is 6. The memory array becomes [1,1,3,4,4,4,1,_,_,_]. We return 6.
        assert_eq!(loc.free_memory(1), 3); // Free all memory units with mID 1. The memory array becomes [_,_,3,4,4,4,_,_,_,_]. We return 3 since there are 3 units with mID 1.
        assert_eq!(loc.allocate(10, 2), -1); // We can not find any free block with 10 consecutive free memory units, so we return -1.
        assert_eq!(loc.free_memory(7), 0); // Free all memory units with mID 7. The memory array remains the same since there is no memory unit with mID 7. We return 0.
    }
}
