// FROM HERE
// https://stackoverflow.com/questions/68453846/how-to-test-the-type-of-an-error-in-boxdyn-error

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
enum VerifyError {
    LinearCombination,
}

impl Error for VerifyError {}

impl Display for VerifyError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn return_result() -> Result<(), Box<dyn Error>> {
    Err(Box::new(VerifyError::LinearCombination))
}

pub fn main() {
    let res = return_result();
    println!("debug print is: {:?}", res);

    // Program does not compile when below line is not commented out
    // assert_eq!(Some(VerifyError::LinearCombination), res.err());
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_error() {
        assert!(super::return_result().is_err());
    }
}
