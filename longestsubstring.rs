use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max:i32 = 0;
        let mut count:i32 = 0;
        let mut i:i32 = 0;
        let len:i32 = s.len() as i32;
        let mut map = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        
        while i < len && ((count + (len - i)) > max){
            let c: char = chars[i as usize];
            if !map.contains_key(&c){
                count += 1;
                map.insert(c,i);
                i += 1;
            }else{
                count = 0;
                if let Some(j) = map.get(&c) {
                    i = *j + 1;              
                }
                map.clear();
            }
            if count > max {
                max = count;
            }
        } 
        max
    }
}