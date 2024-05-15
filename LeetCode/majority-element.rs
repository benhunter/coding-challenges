use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut n = nums.clone();
        n.sort_unstable();
        n.dedup();
        n.clone().into_iter()
            // .inspect(|x| println!("{}", x))
            .map(|x| {
                let c = nums.iter()
                    .filter(|y| *y == &x)
                    .count();
                (x, c)
            })
            // .inspect(|(x,c)| println!("{:?}", c))
            .max_by(|a,b| a.1.cmp(&b.1)).unwrap().0
    }
}