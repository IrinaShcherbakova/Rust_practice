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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode{
            val: 0,
            next: None,
        }));
        
        let mut cur = &mut dummy;
        
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut carry = 0;
        
        loop {  
            // let n1 = p1.as_ref().unwrap();
            // let n2 = p2.as_ref().unwrap();
            
            let mut sum = 0;            
            match (p1, p2){
                (Some(n1), Some(n2)) => {
                    let res = Self::add_nodes(n1.val, n2.val, carry);
                    carry = res.1;
                    sum = res.0;
                    p1 = &n1.next;
                    p2 = &n2.next;
                },
                
                (None, Some(n2)) => {
                    let res = Self::add_nodes(0, n2.val, carry);
                    carry = res.1;
                    sum = res.0;
                    p2 = &n2.next;
                    
                },
                
                (Some(n1), None) => {
                    let res = Self::add_nodes(n1.val, 0, carry);
                    carry = res.1;
                    sum = res.0;
                    p1 = &n1.next;        
                },
                
                (None, None) => {
                    break;
                },                          
            }
            
               let mut new_node = Some(Box::new(ListNode{
                val: sum,
                next: None,
            }));
              
            cur.as_mut().unwrap().next = new_node;
            cur = &mut cur.as_mut().unwrap().next;          
        }
        
        if carry > 0{
            let mut new_node = Some(Box::new(ListNode{
                val: carry,
                next: None,
            }));
            cur.as_mut().unwrap().next = new_node;
        }
             
        dummy.unwrap().next
    }
    
    fn add_nodes(val1: i32, val2: i32, carry: i32) -> (i32, i32){     
        let mut sum = val1 + val2 + carry;
            if sum >= 10 {
                return (sum % 10, 1);
            } else{
                return (sum, 0);
            }
    }
    
    
}
