impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        
        let mut max_candies = 0;
        for candy in candies.iter(){        
            if candy > &max_candies{
                max_candies = *candy;           
            }       
        }
        
        let mut v: Vec<bool> = Vec::new();
        for candy in candies.iter(){
            if candy + extra_candies >= max_candies {
                v.push(true);
            }
            else {
                v.push(false);
            }                    
        }
        
        v
        
    }
}
