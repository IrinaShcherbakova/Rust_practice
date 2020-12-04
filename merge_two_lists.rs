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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        if l1.is_none() && l2.is_none(){
            return None
        }
        
        else if l1.is_none(){
            let node = l2.unwrap();
            return Some(Box::new(ListNode{
                val: node.val,
                next: Self::merge_two_lists(l1, node.next)    
            }))            
        }
                
        else if l2.is_none(){
            let node = l1.unwrap();
            return Some(Box::new(ListNode{
                val: node.val,
                next: Self::merge_two_lists(node.next, l2)    
            }))            
        }
        
        else{
            let node1 = l1.unwrap();
            let node2 = l2.unwrap();
            
            if node1.val <= node2.val{
                return Some(Box::new(ListNode{
                    val: node1.val,
                    next: Self::merge_two_lists(node1.next, Some(node2))    
                }))                    
            }
            
            else{
                return Some(Box::new(ListNode{
                    val: node2.val,
                    next: Self::merge_two_lists(Some(node1), node2.next)    
                }))                 
            }          
        }
    }
}

























