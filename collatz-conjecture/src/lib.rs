pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut num = n;
    let mut count: u64 = 0;
    while num != 1 && count < 1_000_000 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }
        count += 1;
    }
    if num == 1 {
        return Some(count);
    } else {
        return None;
    }
}
