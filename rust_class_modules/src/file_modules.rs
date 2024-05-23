pub fn second_item<T>(slice: &[T]) -> Option<&T> {
    if slice.len() >= 2 {
        Some(&slice[1])
    } else {
        None
    }
}