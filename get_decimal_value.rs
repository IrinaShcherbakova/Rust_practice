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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        
        let mut total = 0;
        let mut cur = head;
        
        loop{        
            match cur {          
                None => break,
                Some(node) => {
                    total = total * 2 + node.val;
                    cur = node.next;
                }           
            }                
        }
        
        total        
    }
}
