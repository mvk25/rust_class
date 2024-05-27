pub fn get_element_by_index(vec: Vec<u32>, index: usize) -> Option<u32> {
    if index < vec.len() {
        Some(vec[index])
    } else {
        None // Index out of bounds
    }
    
}