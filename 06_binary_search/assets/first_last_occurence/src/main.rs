fn lower_bound_binary_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] > target {
            right = mid - 1; // exclude midpoint from the search space
        } else if nums[mid] < target {
            left = mid + 1; // exclude midpoint from the search space
        } else {
            right = mid; // narrow the search space toward the left
        }
    }
    if left < nums.len() && nums[left] == target { Some(left) } else { None }
}

fn upper_bound_binary_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        let mid = 1 + left + (right - left) / 2; // biasing midpoint to the right
        if nums[mid] > target {
            right = mid - 1; // exclude midpoint from the search space
        } else if nums[mid] < target {
            left = mid + 1; // exclude midpoint from the search space
        } else {
            left = mid; // narrow the search space toward the left
        }
    }
    if right < nums.len() && nums[right] == target { Some(right) } else { None }
}

fn first_last_occurrences(nums: &[i32], target: i32) -> (Option<usize>, Option<usize>) {
    (lower_bound_binary_search(nums, target), upper_bound_binary_search(nums, target))
}

fn main() {
    // println!("{:?}", first_last_occurrences(&[1, 2, 3, 4, 4, 4, 5, 6, 6, 8, 9, 10, 11], 4)); // (None, None)
    // println!("{:?}", first_last_occurrences(&[1, 2, 3, 4, 4, 4, 5, 6, 6, 8, 9, 10, 11], 42)); // (None, None)
    println!("{:?}", first_last_occurrences(&[1, 2, 3, 4, 4, 4, 5, 6, 6, 8, 9, 10, 11], 0)); // (None, None)
} // end of local scope OR end of main()       
