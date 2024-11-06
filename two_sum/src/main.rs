use std::collections::HashMap;

struct Solution;

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32)-> Vec<i32> {
        let mut compliments: HashMap<i32, i32> = HashMap::new();
        for (index, num) in nums.iter().enumerate()
        {
            let compliment = target - num;
            if let Some(&compliment_index) = compliments.get(&compliment) {
                return vec![compliment_index, index as i32];
            }
            compliments.insert(*num, index as i32);
        }
        return vec![];
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![3,2,4] ,6));
}
