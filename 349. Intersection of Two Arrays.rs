/*
    Lazy sunday implimentation
    Some cool stuff possible with the intersections method https://doc.rust-lang.org/std/collections/hash_set/struct.Intersection.html

    1ms     83.16%
    2.17    74.74%  
*/

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();

        let mut iter = nums2.iter();
        let mut num2 = iter.next().unwrap(); //unwrap possible as nums2.len > 0
        let mut rtn = vec![];

        for num1 in nums1{
            hile num2 <= &num1{
                if &num1 == num2{
                    rtn.push(num1);
                }
                match iter.next(){
                    Some(n) => num2 = n,
                    None => break,
                }
            }
        }
        rtn.dedup();
        return rtn;
    }
}
