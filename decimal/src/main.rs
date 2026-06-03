use decimal::*;

fn main() {
  /*  let d1 = "123.45";
    let d2 = "67.89";
    let dec1 = Decimal::try_from(d1).expect("Invalid decimal string");
    let dec2 = Decimal::try_from(d2).expect("Invalid decimal string");
    let sum = dec1 + dec2;
    println!("{} + {} = {}", d1, d2, sum); */

    let d3 = "32";
    let dec3 = Decimal::try_from(d3).expect("Invalid decimal string");
    let d4 = "33.1";
    let dec4 = Decimal::try_from(d4).expect("Invalid decimal string");
    let diff = dec3 - dec4;
    println!("{} - {} = {}", d3, d4, diff);



   
}
