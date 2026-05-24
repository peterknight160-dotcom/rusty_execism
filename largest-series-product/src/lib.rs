#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    // The larget series product of a span of {span} digits in {string_digits}
    // Check if the span is longer than the string length

    if span == 0 {
        return Ok(1);
    }
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    // Check for non-digit characters in the string
    for c in string_digits.chars() {
        if !c.is_digit(10) {
            return Err(Error::InvalidDigit(c));
        }
    }
    // All error checks passed, now we can calculate the largest product
    let mut result: u64 = 0;
    string_digits
        .chars()
        .collect::<Vec<char>>()  // Now a vector of chars, we can use windows to get all contiguous spans of the specified length
        .windows(span)
        .for_each(|window| {
            let product: u64 = window
                .iter()  //Break into individual chars in the window
                .map(|c| c.to_digit(10).unwrap() as u64)
                .product();
            // We can keep track of the largest product found so far
            if product > result {
                result = product;
            }
        });

    Ok(result)
}
