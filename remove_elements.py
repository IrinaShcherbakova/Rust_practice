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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head{
            None => None,
            Some(node) => {
                if node.val == val {
                    Self::remove_elements(node.next, val)
                } else{
                    Some(Box::new(ListNode{
                        val: node.val,
                        next: Self::remove_elements(node.next, val)                 
                    }))
                    
                }                         
            }
            
        }
    }
}
