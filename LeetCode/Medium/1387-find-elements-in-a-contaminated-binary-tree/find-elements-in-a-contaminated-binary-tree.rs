use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
struct FindElements {
    values: HashSet<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let root_tree = root.unwrap();
        root_tree.borrow_mut().val = 0;

        let mut deque: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut values: HashSet<i32> = HashSet::new();

        values.insert(0);
        deque.push_back(Rc::clone(&root_tree));

        while let Some(node) = deque.pop_front() {
            let node_val = node.borrow().val;

            if let Some(left) = &node.borrow().left {
                left.borrow_mut().val = node_val * 2 + 1;
                values.insert(left.borrow().val);
                deque.push_back(Rc::clone(left));
            }

            if let Some(right) = &node.borrow().right {
                right.borrow_mut().val = node_val * 2 + 2;
                values.insert(right.borrow().val);
                deque.push_back(Rc::clone(right));
            }
        }

        FindElements { values }
    }

    fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }
}


/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */