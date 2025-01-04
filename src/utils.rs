pub fn bin_search<T: Ord>(arr: &[T], target: T, left_idx: usize, right_idx: usize) -> Option<usize> {

    if left_idx > right_idx {
        return None;
    }
    else if left_idx == right_idx {
        if arr[left_idx] == target {
            return Some(left_idx);
        } else {
            return None;
        }
    }

    let middle_idx: usize = (left_idx + right_idx) / 2;

    if arr[middle_idx] == target {
        return Some(middle_idx);
    } else if target > arr[middle_idx] {
        return bin_search(arr, target, middle_idx + 1, right_idx);
    } else {
        return bin_search(arr, target, left_idx, middle_idx - 1);
    }   
}