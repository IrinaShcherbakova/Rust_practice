use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        
        let mut map: HashMap<i32, i32> = HashMap::new();
        
        for num in nums.iter(){            
            *map.entry(*num).or_insert(0) += 1;         
        }
        
        // println!("map:");
        // for (key, val) in map.iter() {
        //     println!("key: {} val: {}", key, val);
        // }


        let mut res = 0;
        
        for num in nums.iter(){            
            
            if let Some(count) = map.get_mut(&num) {
                //println!("visit {}, count {}", num, *count);
                if *count > 1 {
                    *count -= 1;
                } else{
                    map.remove_entry(&num);                  
                }
            } else{
                continue;
            }
            
            let counter_part = k - num;
            if let Some(count) = map.get_mut(&counter_part) {
                res += 1;
                if *count > 1 {
                    *count -= 1;
                } else{
                    map.remove_entry(&counter_part);                  
                }
            }           
       }
        
        res        
    }
}
