impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations
            .into_iter()
            .map(|x| b',' as i32 - x.into_bytes()[1] as i32)
            .sum()
    }
}
