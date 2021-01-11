impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        
        let mut free_slot = m + n - 1;
        let mut i2 = n - 1;
        let mut i1 = m - 1;
        
        while i2 >= 0 {
            //println!(" i1 {}, i2 {}, free {}", i1, i2, free_slot);
            if (i1 < 0) || (nums2[i2 as usize] >= nums1[i1 as usize]){
                nums1[free_slot as usize] = nums2[i2 as usize];
                i2 -= 1;           
            } else{
                nums1[free_slot as usize] = nums1[i1 as usize];
                i1 -= 1;         
            }
            
            free_slot -= 1;          
        }
        
    }
}
