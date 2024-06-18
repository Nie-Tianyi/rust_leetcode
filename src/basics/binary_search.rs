use std::cmp::Ordering;

/// search target in a sorted array, return the target index
/// # Params
/// * `sorted_array: Vec<T>`: a sorted array in ascending order, T must implement trait `Ord`.
/// * `target: T`: the target to be found, T must implement trait `Ord`.
///
/// # Returns
/// * `Option<usize>`: the index of the array, return `None` if the target does not exist in the array.
///
/// # Examples
/// ```rust
/// use rust_env::basics::binary_search::binary_search;
/// let sorted_array = vec![2,3,8,9];
/// assert_eq!(binary_search(&sorted_array,3),Some(1));
/// assert_eq!(binary_search(&sorted_array,4),None);
/// ```
pub fn binary_search<T: Ord>(sorted_array: &[T], target: T) -> Option<usize> {
    // initialize two pointers, point to the start and the end of the array
    let (mut low, mut high) = (0_usize, sorted_array.len());
    while low <= high {
        // calculate the mid-pointers
        let mid = (low + high) / 2_usize;
        match sorted_array[mid].cmp(&target) {
            Ordering::Less => {
                // if the mid-point's value is less than the target
                // then the target must be on the right half of the array
                low = mid + 1_usize;
            }
            Ordering::Equal => {
                // return index if found
                return Some(mid);
            }
            Ordering::Greater => {
                // vice-verse
                high = mid - 1_usize;
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let sorted_array = vec![2, 3, 8, 9];
        assert_eq!(binary_search(&sorted_array, 9), Some(3));
        assert_eq!(binary_search(&sorted_array, 4), None);
    }
}