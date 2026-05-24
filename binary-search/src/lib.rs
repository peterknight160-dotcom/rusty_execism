

pub fn find2(array: &[i32], key: i32) -> Option<usize> {
    // Binary search implementation
    let len = array.len();
    if len == 0 {
        return None;
    }
    let mut left = 0;
    let mut right = len - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            left = mid + 1;
        } else {
            if mid == 0 {
                break; // Prevents underflow
            }
            right = mid - 1;
        }
    }
    None
}

// Rewrite to take any type that implements PartialEq
pub fn find<T>(array: &[T], key: T) -> Option<usize>
 where
    T: PartialOrd  
{
    let len = array.len();
    if len == 0 {
        return None;
    }
    let mut left = 0;
    let mut right = len - 1; 
    while left <= right {
        let mid = left + (right - left) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            left = mid + 1;
        } else {
            if mid == 0 {
                break; // Prevents underflow
            }
            right = mid - 1;
        }
    }
    None
}