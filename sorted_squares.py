impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {   
        let N = nums.len();
        let mut res = vec![0; N];
        
        let mut index = N-1;
        let mut i = 0;
        let mut j = N-1;
        
        //store elements in num in non-decreasing order regardless their sign      
        while i <= j {
            let left = nums[i].pow(2);
            let right = nums[j].pow(2);
            
            if left >= right {
                res[index] = left;
                i += 1;             
            } else{
                res[index] = right;
                j -= 1;
            }
            
            index -=1;        
        }
        
        res
        
    }
}
