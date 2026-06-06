





use decimal::Decimal;

fn main() {

// Output the sum of-6.5   and +5.
    let a = Decimal::try_from("-6.5").unwrap();
    let b = Decimal::try_from("5").unwrap();
    let sum = b.clone() + a.clone();
    println!("-6.5 + 5 = {}", sum);
    // Now try the same but with the operands reversed to test commutativity
    let sum2 = a + b;
    println!("5 + -6.5 = {}", sum2);

    let c= Decimal::try_from("1.5").unwrap();
    let d = Decimal::try_from("2").unwrap();
    let diff = c.clone() - d.clone();
    println!("1.5 - 2 = {}", diff);
    let diff2 = d - c;
    println!("2 - 1.5 = {}", diff2);    
}
