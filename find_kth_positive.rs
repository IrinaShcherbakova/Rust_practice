impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    
        let mut missing_count = 0;
        let mut last_missing = 0;
        let mut num = 1;
        let mut i = 0;
        let N = arr.len();
        
        loop {
            
            if missing_count == k{
                return last_missing;
            }
            
            else if i < N && arr[i] > num{
                missing_count += 1;
                last_missing = num;            
            }
            
            else if i < N && arr[i] == num{
                i += 1;
            }
            
            else {
                missing_count += 1;
                last_missing = num;  
            }
            
            num += 1;
            
        }
        
    }  
}
