impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        let mut count = 0;
        
        while count < n {
            v.push(nums[count as usize]);
            v.push(nums[(n+count) as usize]);
            count += 1;
        }
        
        v       
    }
}
