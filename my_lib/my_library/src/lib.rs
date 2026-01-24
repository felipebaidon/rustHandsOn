pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn rest(left: u64, right: u64) -> u64{
    left - right
}

pub fn pro( number1: u64, number2: u64) -> u64 {
    number1 * number2
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
