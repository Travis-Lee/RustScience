use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &x) in nums.iter().enumerate() {
        let y = target - x;
        if let Some(&j) = map.get(&y) {
            return vec![j as i32, i as i32];
        }
        map.insert(x, i);
    }

    vec![]
}
