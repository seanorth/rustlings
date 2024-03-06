// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm... Why is this always returning an Ok value?
        match value {
v if v>0 => {Ok(PositiveNonzeroInteger(v as u64))}
            0 => {Err(CreationError::Zero)}
            _ => Err(CreationError::Negative)

        }

    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

#[test]
fn test_chatgpt(){
    let num = -5;

    let r=match num.cmp(&0) {
        std::cmp::Ordering::Greater => {

            "The number is greater than zero."
        }
        std::cmp::Ordering::Equal => {
            "The number is equal to zero."
        }
        std::cmp::Ordering::Less => {
            "The number is less than zero."
        }
    };
    assert_eq!(r,"The number is less than zero.")
}
