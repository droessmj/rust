use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut max, mut count, mut i) = (0,0,0);
        let mut map = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        
        while i < s.len(){
            let c: char = chars[i];
            if !map.contains_key(&c){
                count += 1;
                map.insert(c,i);
                i += 1;
            }else{
                count = 0;
                if let Some(j) = map.get(&c) {
                    i = *j + 1;              
                }
                map = HashMap::new();
            }
            if count > max {
                max = count;
            }
        } 
        max
    }
}