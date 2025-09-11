// This function returns how much icecream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no icecream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day > 23 {
        None
    } else if hour_of_day < 22 {
        Some(5)
    } else {
        Some(0)
    }
}

fn main() {
    println!("{:?}", maybe_icecream(12)); // Some(5)
    println!("{:?}", maybe_icecream(22)); // Some(0)
    println!("{:?}", maybe_icecream(24)); // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12).unwrap(); // отримуємо значення з Some
        assert_eq!(icecreams, 5); // тест пройде
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
