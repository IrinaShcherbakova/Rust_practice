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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        
        let mut stack = Vec::new();
        let size = Self::get_list_size(&head);
        let mut res = Vec::new();      
        for i in (0..size){
            res.push(0);
        }
        
        let mut index = 0;
        let mut cur = head;
        
        while let Some(node) = cur {                  
            
            if stack.is_empty() || stack[stack.len() - 1] >= node.val{
                stack.push(node.val);
            } 
            else{
                let mut res_index = index - 1;
                while (!stack.is_empty()) && (stack[stack.len() - 1] < node.val){
                    while res[res_index] > 0{
                        res_index -= 1;
                    }
                    res[res_index] = node.val;
                    res_index -= 1;
                    stack.pop();
                }
                stack.push(node.val);                
            } 
            
            index += 1;
            cur = node.next;
        }
        
        res
    }
    
    fn get_list_size(mut head: &Option<Box<ListNode>>) -> i32 {
        let mut size = 0;
        while let Some(node) = head{
            size += 1;
            head = &node.next;
        }
        
        size   
    }
    
    
    
    
    
    
}
