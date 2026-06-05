





use decimal::Decimal;

fn main() {
  let five = Decimal::try_from("999").unwrap();
  let point_two = Decimal::try_from("0.9999").unwrap();
  let neg_point_two = Decimal::try_from("-0.2").unwrap();
  let two = neg_point_two.clone() * Decimal::try_from("-50").unwrap();
  let result = five.clone() * point_two.clone();
  println!("{} * {} = {} [ {:?} ]", five   , point_two, result, result);

  println!("{} * {} = {} [ {:?} ]", neg_point_two, Decimal::try_from("-50").unwrap(), two, two);
    

}

    pub fn decimal(input: &str) -> Decimal {
    Decimal::try_from(input).expect("That was supposed to be a valid value")

   
}
