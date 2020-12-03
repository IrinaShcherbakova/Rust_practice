impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counts = vec![0; 101];
        
        for num in nums.iter(){
            counts[*num as usize] += 1;                  
        }
        
        let mut good_pairs = 0;
        for num in counts.iter(){
            good_pairs += Self::num_good_pairs(*num);
        }
        
        good_pairs               
    }
    
    
    fn num_good_pairs(num: i32) -> i32{     
        (num * (num - 1)) / 2          
    }
}
