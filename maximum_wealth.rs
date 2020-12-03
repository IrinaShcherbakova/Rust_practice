impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        
        let mut max_wealth = 0;
        for customer in accounts.iter(){
            let mut cur_wealth = 0;
            for money in customer.iter(){
                cur_wealth += money;
            }
            
            if cur_wealth > max_wealth{
                max_wealth = cur_wealth;            
            }
        }
        
        max_wealth
        
    }
}
