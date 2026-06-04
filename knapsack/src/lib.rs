#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}



pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
  
    // Break this up into a recursive function removes one item at a time and calculates the maximum value for the remaining items and weight.
    if max_weight == 0 || items.is_empty() {
        return 0;   
    } 
    
    
    let item = &items[0];
    if item.weight > max_weight {
        return maximum_value(max_weight, &items[1..]);
    } else {
        let include_value = item.value + maximum_value(max_weight - item.weight, &items[1..]);
        let exclude_value = maximum_value(max_weight, &items[1..]);
        return include_value.max(exclude_value);    
    }    
    
}
