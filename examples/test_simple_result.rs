// FROM HERE
// https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/

fn some_fn() -> Result<bool, String> {
    Ok(true)
}

fn main() {
    println!("{:?}", some_fn());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn result_test() -> Result<(), String> {
        // We can use question mark instead of unwrap.
        // If some_fn() is `Err`, then the test will
        // fail at this line.

        let is_ok = some_fn()?;

        if is_ok {
            Ok(())
        } else {
            // `Err` fails the test
            Err("not ok!".into())
        }
    }
}

// cargo test --example test_simple_result

