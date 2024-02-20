impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut sorted = vec![false; nums.len()+1];
        for val in nums{
            sorted[val as usize] = true;
        }
        //println!("{:?}",sorted);
        return sorted.iter().position(|&r| r == false).unwrap() as i32;
    }
}
