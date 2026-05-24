pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new ();
    let mut c: u32 = 0;
    let mut x: u32 = num;
    let mut d: u32;
    while x > 0 {
        d = x%10;
        digits.push(d);
        x = x /10;
        c+=1;

    }
    // Have list of numbers
    let mut armstrong = 0;
    while let Some (top) = digits.pop() {
        armstrong += top.pow(c);
    }
   
    if armstrong == num {
        return true
    }
    else {
        return false
    }

    //todo!("true if {num} is an armstrong number")
}
