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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut nums = Vec::new();
        let mut cur = &head;
        
        while let Some(node) = cur {
            nums.push(node.val);
            cur = &node.next;         
        }
        
        nums = Self::remove_from_vector(nums);
        
        if nums.len() == 0{
            return None
        }
        
        // make new list
        let mut dummy = Some(Box::new(ListNode{
                                        val: 0,
                                        next: None,
                        }));
        
        let mut cur = &mut dummy;
        for value in nums.iter(){
            let mut node = Some(Box::new(ListNode{
                        val: *value,
                        next: None,
            }));
            
            let mut prev = cur.as_mut().unwrap();
            prev.next = node;
            cur = &mut prev.next;
            
        }
             
        dummy.unwrap().next       
        
    }
    
    
    fn remove_from_vector(nums: Vec<i32>) -> Vec<i32>{
        
        let mut vec = Vec::new();
        for val in nums.iter(){
            if *val != 0 {
                vec.push(*val);
            }
        }
        
        let mut i = 0;
        
        while i < vec.len() {
            
            let mut j = i + 1;
            let mut sub_sum = vec[i];
            let N = vec.len();
            
            while j < N {
                sub_sum += vec[j];
                if sub_sum == 0{
                    if i > 0{
                        let mut temp = vec[0..i].to_vec();
                        temp.append(&mut vec[j+1..N].to_vec());
                        vec = temp;
                    } else{
                        vec = vec[j+1..N].to_vec();
                    }
                    i -= 1;
                    break;
                }
                j += 1;              
            }
            
            i += 1;
        }
        
        vec       
    }
    
}
