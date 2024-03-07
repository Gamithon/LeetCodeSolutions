/*
    Runtime     0ms     100%
    Memory      2.04MB  82.01%
*/

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //count nodes
        let mut count = 0;
        let mut node = &head;
        loop{
            match node{
                Some(n) => {
                    node = &n.next;
                    count+=1;
                }
                None => break,
            }
        }
        count = count/2;
        //move n nodes
        let mut node = head.clone();
        for n in 0..count{
            match node{
                Some(n) => node = n.next,                
                None => break,
            }
        }
        return node;
    }
}
