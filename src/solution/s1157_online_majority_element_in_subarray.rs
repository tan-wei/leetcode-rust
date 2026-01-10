/**
 * [1157] Online Majority Element In Subarray
 *
 * Design a data structure that efficiently finds the majority element of a given subarray.
 * The majority element of a subarray is an element that occurs threshold times or more in the subarray.
 * Implementing the MajorityChecker class:
 *
 * 	MajorityChecker(int[] arr) Initializes the instance of the class with the given array arr.
 * 	int query(int left, int right, int threshold) returns the element in the subarray arr[left...right] that occurs at least threshold times, or -1 if no such element exists.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MajorityChecker", "query", "query", "query"]
 * [[[1, 1, 2, 2, 1, 1]], [0, 5, 4], [0, 3, 3], [2, 3, 2]]
 * Output
 * [null, 1, -1, 2]
 * Explanation
 * MajorityChecker majorityChecker = new MajorityChecker([1, 1, 2, 2, 1, 1]);
 * majorityChecker.query(0, 5, 4); // return 1
 * majorityChecker.query(0, 3, 3); // return -1
 * majorityChecker.query(2, 3, 2); // return 2
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 2 * 10^4
 * 	1 <= arr[i] <= 2 * 10^4
 * 	0 <= left <= right < arr.length
 * 	threshold <= right - left + 1
 * 	2 * threshold > right - left + 1
 * 	At most 10^4 calls will be made to query.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/online-majority-element-in-subarray/
// discuss: https://leetcode.com/problems/online-majority-element-in-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/online-majority-element-in-subarray/solutions/761571/rust-translated/

struct MajorityChecker {
    indices: std::collections::HashMap<i32, Vec<i32>>,
    tree: Vec<Vec<i32>>,
    size: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let size = arr.len();
        let mut tree = vec![vec![0, 0]; size * 4];
        Self::build_tree(&arr, &mut tree, 0, 0, (size - 1) as i32);
        let mut indices = std::collections::HashMap::<i32, Vec<i32>>::new();
        for i in 0..size {
            indices.entry(arr[i]).or_default().push(i as i32);
        }

        Self {
            tree,
            indices,
            size: size as i32,
        }
    }

    fn query(&mut self, left: i32, right: i32, threshold: i32) -> i32 {
        if self.size == 0 {
            return -1;
        }
        let node = Self::find_max(&mut self.tree, 0, 0, self.size - 1, left, right);
        if node[1] == 0 {
            return -1;
        }
        let idx_arr = self.indices.get(&node[0]).unwrap();
        let it1 = Self::lower_bound(&idx_arr, &left);
        let it2 = Self::upper_bound(&idx_arr, &right);
        let cnt = it2 - it1;
        if cnt >= threshold { node[0] } else { -1 }
    }

    fn find_max(
        tree: &mut Vec<Vec<i32>>,
        root: i32,
        left: i32,
        right: i32,
        left_limited: i32,
        right_limited: i32,
    ) -> Vec<i32> {
        let node = tree[root as usize].clone();
        if left >= left_limited && right <= right_limited {
            return node;
        }
        let mid = (left + right) / 2;
        let rl = root * 2 + 1;
        let rr = rl + 1;
        if mid < left_limited {
            return Self::find_max(tree, rr, mid + 1, right, left_limited, right_limited);
        }
        if mid + 1 > right_limited {
            return Self::find_max(tree, rl, left, mid, left_limited, right_limited);
        }
        let mut t = vec![0, 0];
        return Self::merge(
            &mut t,
            Self::find_max(tree, rl, left, mid, left_limited, right_limited),
            Self::find_max(tree, rr, mid + 1, right, left_limited, right_limited),
        );
    }

    fn build_tree(arr: &[i32], tree: &mut Vec<Vec<i32>>, root: i32, left: i32, right: i32) {
        if left == right {
            tree[root as usize][0] = arr[left as usize];
            tree[root as usize][1] = 1;
            return;
        }
        let mid = (left + right) / 2;
        let rl = root * 2 + 1;
        let rr = rl + 1;
        Self::build_tree(arr, tree, rl, left, mid);
        Self::build_tree(arr, tree, rr, mid + 1, right);
        let l_tree = tree[rl as usize].clone();
        let r_tree = tree[rr as usize].clone();
        Self::merge(&mut tree[root as usize], l_tree, r_tree);
    }

    fn merge(root: &mut Vec<i32>, left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        if left[0] == right[0] {
            root[0] = left[0];
            root[1] = left[1] + right[1];
        } else if left[1] > right[1] {
            root[0] = left[0];
            root[1] = left[1] - right[1];
        } else {
            root[0] = right[0];
            root[1] = right[1] - left[1];
        }
        root.clone()
    }

    fn lower_bound(v: &[i32], e: &i32) -> i32 {
        let mut left = 0;
        let mut right = v.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if v[mid] >= *e {
                right = mid;
            } else {
                left += 1
            }
        }
        left as i32
    }

    fn upper_bound(v: &[i32], e: &i32) -> i32 {
        let mut left = 0;
        let mut right = v.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if v[mid] > *e {
                right = mid;
            } else {
                left += 1
            }
        }
        right as i32
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1157_example_1() {
        let mut majority_checker = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
        assert_eq!(majority_checker.query(0, 5, 4), 1); // return 1
        assert_eq!(majority_checker.query(0, 3, 3), -1); // return -1
        assert_eq!(majority_checker.query(2, 3, 2), 2); // return 2
    }
}
