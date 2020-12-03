impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        let mut prev = 0;
        
        for num in nums.iter(){
            prev += num;
            v.push(prev);
        }
        
        v      
    }
}
