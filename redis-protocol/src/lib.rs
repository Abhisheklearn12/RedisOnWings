/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let result = redis_protocol::add(2, 3);
/// assert_eq!(result, 5);
/// ```

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
