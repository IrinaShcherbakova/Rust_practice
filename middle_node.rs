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
        
        let mut length = 0;
        let mut cur = &head;
        
        loop {
            match cur{
                None => break,
                Some(node) => {
                    length += 1;
                    cur = &node.next;
                }
            }
        }

        let mid = length / 2;
        let mut cur = head;
        let mut i = 0;
        
        while i < mid {
            match cur{
                None => break,
                Some(node) => cur = node.next,
            }
            i += 1;          
        }
        
        cur
        
    }
}
