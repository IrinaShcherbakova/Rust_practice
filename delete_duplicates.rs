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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
       match head{
           None => None,
           Some(node) => Some(Box::new(ListNode{
               val: node.val,
               next: Self::delete_node(node.next, node.val),                
           }))
        }
 
    }
    
    
    fn delete_node(head: Option<Box<ListNode>>, old_val: i32) -> Option<Box<ListNode>> {
    
        match head{
           None => None,
           Some(node) => {          
               if node.val == old_val {
                   Self::delete_node(node.next, old_val)                
               }
               else{
               Some(Box::new(ListNode{
                    val: node.val,
                    next: Self::delete_node(node.next, node.val)})) 
               }
            }                
        }      
    }
}
