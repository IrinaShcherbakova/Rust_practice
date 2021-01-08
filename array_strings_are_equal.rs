impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        
        let string1 = Self::construct_str(word1);
        let string2 = Self::construct_str(word2);
        
        return string1 == string2;
    }
    
    
    
    fn construct_str(words: Vec<String>) -> String{
        
        let mut res = String::new();
        for word in words.iter(){
            res.push_str(word);
        }
        
        res      
    }
}
