use std::collections::HashMap; // import HashMap library
// use std::vector;

#[derive(Debug)]
struct Solution {

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, usize> = HashMap::new();
        // create a hashmap to store num, index pair
        for (i, num) in nums.iter().enumerate() { // iterate over the indices and elements
            let complement = target - num; // take the difference of a number from target
            if let Some(&j) = num_map.get(&complement) {
                return vec![j as i32, i as i32];
                // in best case, we find the difference stored in the hashmap and
                // thus found the pair of elements on those indicies
            }
            num_map.insert(*num, i); // otherwise we store the pair of element and index
        }

        vec![] // No solution found so return empty vector
    }
}

#[test]
fn test_two_sums() {
    let result = Solution::two_sum(vec![3, 4, 5], 7);
    assert!(result == vec![0, 1]);
}

#[test]
fn test_not_same_index() {
    let result = Solution::two_sum(vec![1, 5, 9, 5], 10);
    assert!(result != vec![1, 1]);
}

#[test]
fn test_empty_vec() {
    let result = Solution::two_sum(vec![1], 5);
    assert!(Vec::is_empty(&result));
}

fn main() {
    println!("Solution and Test for Two Sums");
}