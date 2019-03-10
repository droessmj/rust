impl Solution {
    pub fn count_and_say(n: i32) -> String {          
        let mut result = String::new();
        for i in 1..n+1 {
            if i == 1 {
                result = String::from("1");
            }else{
                result = Solution::say_string(result);
            }
        }
        result
    }
    
    pub fn say_string(s: String) -> String {
        let mut new = String::new();
        let mut chars = s.chars();        
        
        //not sure on safety of unwrap here, but i 
        //can expect all strings inbound to have at least one
        //char due to the base rule in above...
        let mut cur:char = chars.next().unwrap();
        let mut count = 1;
        
        while let Some(c) = chars.next(){
            if c == cur {
                //subsequent passes
                count += 1;
            } else {
                //change in char, process previous
                new.push_str(&format!("{}{}",count,cur));
                count = 1;
                cur = c;
            }
        }
        //process final line
        new.push_str(&format!("{}{}",count,cur));
        
        return new
    }
}