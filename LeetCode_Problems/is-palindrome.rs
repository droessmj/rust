// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        
        if head.is_none(){
            return true
        }
        
        let mut list:Vec<i32> = vec![];
        
        let mut copy = head;
        while let Some(v) = copy{
            list.push(v.val);
            copy = v.next;
        }

        let mut i = 0;
        let mut j = list.len()-1;
        while i < j{
            if list[i] != list[j] {
                return false
            }
            i += 1; j -= 1;
        }

        if i == j || i > j {
            return true
        }

        return false
    }
}