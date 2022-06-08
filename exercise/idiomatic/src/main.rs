fn count_to_5() -> i32 {
    let mut count = 0;
    loop {
        if count >= 5 {
            return count;
        }
        count += 1;
    }
}
fn main() {
    println!("I can count to {}", count_to_5());
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
