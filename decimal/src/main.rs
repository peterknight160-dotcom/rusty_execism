


use decimal::Decimal;

fn main() {
   /*  let d1 = "12";
    let d2 = "1000.1";
    let dec1 = Decimal::try_from(d1).expect("Invalid decimal string");
    let dec2 = Decimal::try_from(d2).expect("Invalid decimal string");
    let sum = dec1 + dec2;
    println!("{} + {} = {}", d1, d2, sum); 

     let d3 = "13.1";
    let dec3 = Decimal::try_from(d3).expect("Invalid decimal string");
    let d4 = "999";
    let dec4 = Decimal::try_from(d4).expect("Invalid decimal string");
    let diff = dec3 - dec4;
    println!("{} - {} = {}", d3, d4, diff); 
    let dec1a = Decimal::try_from(d1).expect("Invalid decimal string");
    let dec3a = Decimal::try_from(d3).expect("Invalid decimal string");
    let product = dec1a * dec3a;
    println!("{} * {} = {}", d1, d3, product); 
    let dec8 = Decimal::try_from("123").expect("Invalid decimal string");
    let dec9 = Decimal::try_from("223").expect("Invalid decimal string");
    let product2 = dec8.clone() * dec9;
    println!("{} * {} = {}", "123", "223", product2);
*/
 let dec8 = Decimal::try_from("123").expect("Invalid decimal string");
    // Loop from 10 to 22 and multiply each number by 123, and print the result
    for i in 10..=22 {
        let dec_i = Decimal::try_from(i.to_string().as_str()).expect("Invalid decimal string");
        let product = dec8.clone() * dec_i;
        println!("{} * {} = {}", "123", i, product);
  
    } 
    for i in 1..=20 {
        let dec_i = Decimal::try_from((i*5).to_string().as_str()).expect("Invalid decimal string");
     
        println!("i is {}  {:?} ", dec_i, dec_i);
    }
    // Generate number 0.01 to 1.00 in increments of 0.01, and print each number
    for i in 0..=90 {
        let dec_i = Decimal::try_from(format!("{:.999}", i as f64 * 1./3. ).as_str()).expect("Invalid decimal string");
        
        
     
        println!("i is {}   ", dec_i, );
    }
 
    let dec1 = Decimal::try_from("123.45").expect("Invalid decimal string");
    println!("dec1 is {}  {:?} ", dec1, dec1);
let dec1     = Decimal::try_from("0.045").expect("Invalid decimal string");
    println!("dec1 is {}  {:?} ", dec1, dec1);
 let dec1     = Decimal::try_from("0.0000").expect("Invalid decimal string");
    println!("dec1 is {}  {:?} ", dec1, dec1);

   
}
