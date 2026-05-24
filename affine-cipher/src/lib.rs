/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    // Implement the encoding logic here, but for now we just return an empty string.
    let mut cyphertext = String::new();
        for c in plaintext.chars().map(|c| c.to_ascii_lowercase()) {
            if c.is_ascii_alphabetic() {
                let base = b'a';
                let x = (c as u8 - base) as i32;
                let encoded_char = ((a * x + b) % 26) as u8 + base;
                cyphertext.push(encoded_char as char);
            } else {
                if c.is_ascii_digit() {
                    cyphertext.push(c);
                }
            }
        }
    // Add spaces every 5 characters in the cyphertext
    let mut formatted_cyphertext = String::new();
    for (i, c) in cyphertext.chars().enumerate() {
        if i > 0 && i % 5 == 0 {
            formatted_cyphertext.push(' ');
        }
        formatted_cyphertext.push(c);
    }
    Ok(formatted_cyphertext)
   
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    // Implement the decoding logic here, but for now we just return an empty string.
    let mut plaintext = String::new();
    let a_inv = mod_inverse(a, 26);
    for c in ciphertext.chars().map(|c| c.to_ascii_lowercase()) {
        if c.is_ascii_alphabetic() {
            let base = b'a';
            let y = (c as u8 - base) as i32;
            let mut decoded_int = (a_inv * (y - b + 26)) % 26;
            if decoded_int < 0 {
                decoded_int += 26;
            }
            let decoded_char = (decoded_int as u8 + base) as char;
            plaintext.push(decoded_char);
        } else {
            if c.is_ascii_digit() {
                plaintext.push(c);
            }
        }
    }
    Ok(plaintext)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}
fn mod_inverse(a: i32, m: i32) -> i32 {
    let mut m0 = m;
    let mut y = 0;
    let mut x = 1;

    if m == 1 {
        return 0;
    }

    let mut a = a % m;
    while a > 1 {
        // q is quotient
        let q = a / m0;
        let mut t = m0;

        // m is remainder now, process same as Euclid's algorithm
        m0 = a % m0;
        a = t;
        t = y;

        // Update y and x
        y = x - q * y;
        x = t;
    }

    // Make x positive
    if x < 0 {
        x += m;
    }

    x
}
