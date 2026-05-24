pub fn egg_count(display_value: u32) -> u32 {
   

    let mut count : u32 = 0;
    let mut value :u32 = display_value ;
    while value > 0 {
        if value % 2 == 1 {
            count +=1 ;
        }
        value /=2;
    }

    count 
}
