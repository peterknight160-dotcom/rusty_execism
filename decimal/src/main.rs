use decimal::*;
fn main() {


    let t5 = "123400.0000";
    let t6 = "876543.99"; 
      let e = Decimal::try_from(t5).unwrap();
    let f = Decimal::try_from(t6).unwrap();
   
    println!("e: {}, f: {}", e, f);
  /*   let mut sum = add_unsigned(&e, &f);
    
    println!("Sum of e and f: {}", sum);
    sum = add_unsigned(&f, &e);
    println!("Sum of f and e: {}", sum); */

   let      sum = e+f ;
    println!("Sum of e and f: {}", sum);
}
