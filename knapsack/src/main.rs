use knapsack::*;

pub fn main() {
    let max_weight = 3 ;
  let items = [
        Item {
            weight: 2,
            value: 5,
        },
        
        Item {
            weight: 1,
            value: 5,
        },
        Item {
            weight: 1,
            value: 7,
        },
           ];
    let output = maximum_value(max_weight, &items);
    println!("Maximum value: {}", output);


}  