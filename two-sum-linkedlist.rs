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
    
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        match(l1,l2){
            
            (Some(mut v1),None) | (None, Some(mut v1))=> {
                if v1.val > 9{
                    v1.val = v1.val - 10;
                    let x = v1.clone();
                    if let Some(mut y) = x.next{
                        y.val += 1;
                        v1.next = Solution::add_two_numbers(Some(y),None);
                    }else{
                        v1.next = Some(Box::new(ListNode::new(1)));
                    }
                }
                Some(v1)
            },           
            (Some(mut v1),Some(mut v2)) => {
                let mut out = v1.val + v2.val;
                let mut l1next = v1.next.take();
                let mut l2next = v2.next.take();
                if out > 9{
                    out = out-10;
                    match(&mut l1next,&mut l2next){
                        (Some(ref mut l1nextNode),_) => {
                            l1nextNode.val += 1;
                            let x = l1nextNode.clone();
                            l1next = Solution::add_two_numbers(Some(x),None);
                        },
                        (None,Some(ref mut l2nextNode))  => {                          
                            l2nextNode.val += 1;
                            let x = l2nextNode.clone();
                            l2next = Solution::add_two_numbers(Some(x),None);
                        },
                        (None,None) => l1next = {
                            Some(Box::new(ListNode::new(1)))
                        },
                        _ => (),
                    }
                }
                let mut head = ListNode::new(out);
                head.next = Solution::add_two_numbers(l1next, l2next);               
                Some(Box::new(head))
            },
            
            _ => None,
        }
    }
}