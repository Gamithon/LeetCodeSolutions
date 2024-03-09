/*
    3ms     100%
    4.00MB  52.38%
*/


impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut iter1 = nums1.iter();
        let mut iter2 = nums2.iter();
        let mut num1 = iter1.next().unwrap(); // Only works because length >1
        let mut num2 = iter2.next().unwrap();
        loop{
            if num1 == num2{
                return *num1;
            }
            if num1 < num2{
                match iter1.next(){
                    Some(n) => num1=n,
                    None => return -1
                }
            }
            if num1 > num2{
                match iter2.next(){
                    Some(n) => num2=n,
                    None => return -1
                }
            }
        }
    }
    
}
