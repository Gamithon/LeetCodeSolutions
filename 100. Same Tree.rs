/*
    Not the best implimentation
    1 ms 18.56%
    2.16 MB 61.34%
*/



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
use std::rc::Rc;
use std::cell::RefCell;

pub fn check_node(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool{
        if let Some(node1) = p {//&& let Some(node2) = q{
            if let Some(node2) = q{
                let node1 = node1.borrow();
                let node2 = node2.borrow();
                if &node1.val != &node2.val{
                    return false;
                }
                else{
                    return check_node(&node1.left, &node2.left) && check_node(&node1.right, &node2.right);
                }
            }
            else{
                return false;
            }
        }
        else if *q == None{
            return true;
        }
        else{
            return false;
        }
    }

impl Solution {
    /*
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        check_node(&p,&q)
    }
    */
    //lol there was an eq trait
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p == q
    }
}


//A much cleaner version of what I was trying to accomplish
//Lesson here is that you can match two symbols at once.
/*
match (p, q) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(ref m), Some(ref n)) => {
                let m = m.borrow();
                let n = n.borrow();
                m.val == n.val && Self::is_same_tree(n.left.clone(), m.left.clone()) && Self::is_same_tree(n.right.clone(), m.right.clone()) 
            }
        }
*/
