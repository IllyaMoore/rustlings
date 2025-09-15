use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    let qty = item_quantity.parse::<i32>()?;
    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    println!("{:?}", total_cost("34"));       // Ok(171)
    println!("{:?}", total_cost("beep boop")); // Err(ParseIntError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
