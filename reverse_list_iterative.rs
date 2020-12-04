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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
       //iterative with 2 pointers
        let mut a: Option<Box<ListNode>> = None;
        let mut b = head;
        
        loop {          
            match b{
                None => {
                    break;
                },
                Some(mut node) => {
                    b = node.next; 
                    node.next = a;
                    a = Some(node);          
                },             
            }           
        }      
        a        
    }
}



























