#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        if value < 0 {
            Err(CreationError::Negative)
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Ok(Self(value as u64))
        }
    }
}

fn main() {
    println!("{:?}", PositiveNonzeroInteger::new(10));  // Ok(PositiveNonzeroInteger(10))
    println!("{:?}", PositiveNonzeroInteger::new(0));   // Err(Zero)
    println!("{:?}", PositiveNonzeroInteger::new(-5));  // Err(Negative)
}
