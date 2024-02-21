/*
    0ms runtime!
    2.70 MB

    This would be faster using real bits instead of Vec<bool>
    Could be considered more general as it supports any number of bits.
*/

use core::cmp;

pub fn convert_i32(tenbase:i32) -> Vec<bool>{
    let mut copy = tenbase;
    let mut vec:Vec<bool> = vec![];
    while copy>0{
        if copy%2 == 1{
            vec.push(true);
        }
        else{
            vec.push(false);
        }
        copy=copy/2;
    }
    return vec;
}
pub fn convert_and(a:Vec<bool>,b:Vec<bool>,skip:usize) -> i32{
    let mut sum = 0;    
    //println!("{:?} , {:?} , skip:{}",a,b,skip);
    //min bits prevents and operations without values
    for n in skip..cmp::min(a.len(),b.len()){
        if a[n] && b[n]{
            sum+= 2_i32.pow(n as u32);
        }
    }
    sum as i32
}

impl Solution {   
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let skip = (left - right).abs();
        //First n bits can be skipped as they will always flip incrementing up to the highest value
        let skip = (skip as f64).log2().ceil() as usize;
        convert_and(convert_i32(left) ,convert_i32(right),skip)
    }
}
