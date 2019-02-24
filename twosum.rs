use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut i = 0;
        for (i,x) in nums.iter().enumerate(){
            map.insert(x,i);
        }
        
        i = 0;
        for (i, x) in nums.iter().enumerate(){
            let complement = target - x;
            match map.get(&complement) {
                Some(&res) if res != i =>  return vec![i as i32, res as i32],
                Some(_) | None => continue,
            }
        }
        
        vec![99,99]
    }
}