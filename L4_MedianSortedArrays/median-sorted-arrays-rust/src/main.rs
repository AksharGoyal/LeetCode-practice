struct Solution {

}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums_merged: Vec<i32> =  Vec::new(); // vectors will allow us to extend our list of elements as needed
        let mut i = 0;
        let mut j = 0;
        let m = nums1.len();
        let n = nums2.len();
    
        // We will merged the two arrays while comparing
        while (i < m) && (j < n) {
            if nums1[i] < nums2[j] { nums_merged.push(nums1[i]); i += 1; } else { nums_merged.push(nums2[j]); j += 1; }
        }
    
        // As both the vectors can have variable length, we need to add remaining elements as well
        while i < m { 
            nums_merged.push(nums1[i]);
            i += 1;
        }
    
        while j < n { 
            nums_merged.push(nums2[j]);
            j += 1;
        }
    
        // After getting all the elements, we will try to find the median
        let mid_idx = nums_merged.len() / 2;
        /*
        median can be calculated in 2 ways depending on the even-ness of length of the merged array
        if there are even number of elements, median can be calculated as mean of middle 2 elements (median of [1, 2, 3, 4] is 2.5)
        else we just pick the middle element as median (median of [1, 2, 3] is 2)
        */
        let median = if nums_merged.len() % 2 == 1 { nums_merged[mid_idx] as f64} else {(nums_merged[mid_idx - 1] + nums_merged[mid_idx]) as f64 / 2.0};
        
        return median;
        }
    }

#[test]
fn test_one_empty_list() {
    let l1 = vec![1, 2, 3, 4];
    let l2 = Vec::new();
    assert_eq!(Solution::find_median_sorted_arrays(l1, l2), 2.5);
}

// #[test]
// fn test_two_empty_list() {
//     let l1 = Vec::new();
//     let l2 = Vec::new();
//     assert_eq!(Solution::find_median_sorted_arrays(l1, l2), 2.5);
// }

#[test]
fn test() {
    let l1 = vec![1, 3, 7, 9];
    let l2 = vec![2, 6, 8];
    assert_eq!(Solution::find_median_sorted_arrays(l1, l2), 6.0);
}

fn main() {
    println!("Solution and Test for Median Sorted Arrays");
}
