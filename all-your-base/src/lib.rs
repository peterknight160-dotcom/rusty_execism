#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // First job - invalid input
    // Part 1a - Invalid input or output base
    // Part 1b - Invalid digit in input

    let check= check_bases ( from_base, to_base);
    if check.is_err() { 
        return check ;
    }
   
    // Convert Input into a internal number 
    let mut input: u32 = 0 ;
    for iter_number in number.iter() {
        if *iter_number >= from_base {
            return Err ( Error::InvalidDigit(*iter_number)) ;
        }
        input *= from_base ;
        input += iter_number; 
    }
     if  input == 0   {  // Deals with case when either no input or when input is 1 or more 0's
        return  Ok ( vec![0]);
    }
    // Convert input number to the output vector
    let mut output : Vec <u32> = Vec::new();
    while input > 0 {
        output.push (input % to_base) ;
        input /= to_base ;

    }
    // Have result but reversed 
    output.reverse() ;
    
    Ok(output)
}

fn check_bases ( from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err( Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err ( Error::InvalidOutputBase)
    }
    Ok ( vec![0])
}