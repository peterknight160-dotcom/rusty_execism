#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    // Input is any arary of 32 bit unsigned integers, output is a vector of bytes 
    let mut result: Vec<u8> = Vec::new();
    for &value in values {
        let mut bytes: Vec<u8> = Vec::new();
        let mut n = value ;
        loop {
            let byte = (n & 0x7F) as u8; // Get the last 7 bits
            bytes.push(byte);
            n >>= 7; // Shift right by 7 bits
            if n == 0 {
                break;
            }
        }
        bytes.reverse(); // Reverse to get the correct order
        for i in 0..bytes.len() - 1 {
            bytes[i] |= 0x80; // Set the continuation bit for all but the last byte
        }
        result.extend(bytes);
    }
    result
    //todo!("Convert the values {values:?} to a list of bytes")
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result: Vec<u32> = Vec::new();
    let mut current_value: u32 = 0;
    for &byte in bytes {
        current_value = (current_value << 7) | (byte as u32 & 0x7F); // Shift left and add the last 7 bits
        if byte & 0x80 == 0   { // If the continuation bit is not set, we have a complete number
            result.push(current_value);
            current_value = 0; // Reset for the next number
        }
    }
    if current_value != 0  || result.is_empty() {  
        return Err(Error::IncompleteNumber); 
    }
   
    Ok(result)
    //todo!("Convert the bytes {bytes:?} to a list of numbers")
}
