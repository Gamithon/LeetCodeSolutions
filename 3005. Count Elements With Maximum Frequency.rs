use std::collections::HashMap;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut instances:HashMap<i32,i32> = Default::default();;
        for num in nums{
            *instances.entry(num).or_insert(0) +=1;
        }
        //println!("{:?}",instances);
        //find max instance
        let max = instances.iter().max_by_key(|v| v.1).unwrap();

        //return values with that many
        instances.iter().filter(|n| n.1==max.1).map(|n| n.1).sum()
        //return *max.unwrap().1;
    }
}
