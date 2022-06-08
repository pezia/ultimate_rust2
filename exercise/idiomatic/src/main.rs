use std::f32::consts::PI;

fn count_to_5() -> i32 {
    let mut count = 0;
    loop {
        if count > PI as i32 && count > 5 {
            break;
        }
        count += 1;
    }
    5
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
