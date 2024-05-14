/**
 * [1600] Throne Inheritance
 *
 * A kingdom consists of a king, his children, his grandchildren, and so on. Every once in a while, someone in the family dies or a child is born.
 * The kingdom has a well-defined order of inheritance that consists of the king as the first member. Let's define the recursive function Successor(x, curOrder), which given a person x and the inheritance order so far, returns who should be the next person after x in the order of inheritance.
 *
 * Successor(x, curOrder):
 *     if x has no children or all of x's children are in curOrder:
 *         if x is the king return null
 *         else return Successor(x's parent, curOrder)
 *     else return x's oldest child who's not in curOrder
 *
 * For example, assume we have a kingdom that consists of the king, his children Alice and Bob (Alice is older than Bob), and finally Alice's son Jack.
 * <ol>
 * 	In the beginning, curOrder will be ["king"].
 * 	Calling Successor(king, curOrder) will return Alice, so we append to curOrder to get ["king", "Alice"].
 * 	Calling Successor(Alice, curOrder) will return Jack, so we append to curOrder to get ["king", "Alice", "Jack"].
 * 	Calling Successor(Jack, curOrder) will return Bob, so we append to curOrder to get ["king", "Alice", "Jack", "Bob"].
 * 	Calling Successor(Bob, curOrder) will return null. Thus the order of inheritance will be ["king", "Alice", "Jack", "Bob"].
 * </ol>
 * Using the above function, we can always obtain a unique order of inheritance.
 * Implement the ThroneInheritance class:
 *
 * 	ThroneInheritance(string kingName) Initializes an object of the ThroneInheritance class. The name of the king is given as part of the constructor.
 * 	void birth(string parentName, string childName) Indicates that parentName gave birth to childName.
 * 	void death(string name) Indicates the death of name. The death of the person doesn't affect the Successor function nor the current inheritance order. You can treat it as just marking the person as dead.
 * 	string[] getInheritanceOrder() Returns a list representing the current order of inheritance excluding dead people.
 *
 *  
 * Example 1:
 *
 * Input
 * ["ThroneInheritance", "birth", "birth", "birth", "birth", "birth", "birth", "getInheritanceOrder", "death", "getInheritanceOrder"]
 * [["king"], ["king", "andy"], ["king", "bob"], ["king", "catherine"], ["andy", "matthew"], ["bob", "alex"], ["bob", "asha"], [null], ["bob"], [null]]
 * Output
 * [null, null, null, null, null, null, null, ["king", "andy", "matthew", "bob", "alex", "asha", "catherine"], null, ["king", "andy", "matthew", "alex", "asha", "catherine"]]
 * Explanation
 * ThroneInheritance t= new ThroneInheritance("king"); // order: king
 * t.birth("king", "andy"); // order: king > andy
 * t.birth("king", "bob"); // order: king > andy > bob
 * t.birth("king", "catherine"); // order: king > andy > bob > catherine
 * t.birth("andy", "matthew"); // order: king > andy > matthew > bob > catherine
 * t.birth("bob", "alex"); // order: king > andy > matthew > bob > alex > catherine
 * t.birth("bob", "asha"); // order: king > andy > matthew > bob > alex > asha > catherine
 * t.getInheritanceOrder(); // return ["king", "andy", "matthew", "bob", "alex", "asha", "catherine"]
 * t.death("bob"); // order: king > andy > matthew > <s>bob</s> > alex > asha > catherine
 * t.getInheritanceOrder(); // return ["king", "andy", "matthew", "alex", "asha", "catherine"]
 *
 *  
 * Constraints:
 *
 * 	1 <= kingName.length, parentName.length, childName.length, name.length <= 15
 * 	kingName, parentName, childName, and name consist of lowercase English letters only.
 * 	All arguments childName and kingName are distinct.
 * 	All name arguments of death will be passed to either the constructor or as childName to birth first.
 * 	For each call to birth(parentName, childName), it is guaranteed that parentName is alive.
 * 	At most 10^5 calls will be made to birth and death.
 * 	At most 10 calls will be made to getInheritanceOrder.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/throne-inheritance/
// discuss: https://leetcode.com/problems/throne-inheritance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct ThroneInheritance {
    king: String,
    dead: std::collections::HashMap<String, bool>,
    children: std::collections::HashMap<String, Vec<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ThroneInheritance {
    fn new(king_name: String) -> Self {
        Self {
            king: king_name,
            dead: std::collections::HashMap::new(),
            children: std::collections::HashMap::new(),
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.children
            .entry(parent_name)
            .or_insert_with(Vec::new)
            .push(child_name);
    }

    fn death(&mut self, name: String) {
        self.dead.insert(name, true);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut order = Vec::new();
        self.dfs_helper(&self.king, &mut order);
        order
    }

    fn dfs_helper(&self, name: &str, order: &mut Vec<String>) {
        if !self.dead.contains_key(name) {
            order.push(name.to_string());
        }
        if let Some(children) = self.children.get(name) {
            for child in children {
                self.dfs_helper(child, order);
            }
        }
    }
}

/**
 * Your ThroneInheritance object will be instantiated and called as such:
 * let obj = ThroneInheritance::new(kingName);
 * obj.birth(parentName, childName);
 * obj.death(name);
 * let ret_3: Vec<String> = obj.get_inheritance_order();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1600_example_1() {
        let mut t = ThroneInheritance::new("king".to_string()); // order: king
        t.birth("king".to_string(), "andy".to_string()); // order: king > andy
        t.birth("king".to_string(), "bob".to_string()); // order: king > andy > bob
        t.birth("king".to_string(), "catherine".to_string()); // order: king > andy > bob > catherine
        t.birth("andy".to_string(), "matthew".to_string()); // order: king > andy > matthew > bob > catherine
        t.birth("bob".to_string(), "alex".to_string()); // order: king > andy > matthew > bob > alex > catherine
        t.birth("bob".to_string(), "asha".to_string()); // order: king > andy > matthew > bob > alex > asha > catherine
        assert_eq!(
            t.get_inheritance_order(),
            vec_string![
                "king",
                "andy",
                "matthew",
                "bob",
                "alex",
                "asha",
                "catherine"
            ]
        ); // return ["king", "andy", "matthew", "bob", "alex", "asha", "catherine"]
        t.death("bob".to_string()); // order: king > andy > matthew > <s>bob</s> > alex > asha > catherine
        assert_eq!(
            t.get_inheritance_order(),
            vec_string!["king", "andy", "matthew", "alex", "asha", "catherine"]
        ); // return ["king", "andy", "matthew", "alex", "asha", "catherine"]
    }
}
