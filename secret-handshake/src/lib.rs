const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];
// Actions are bit 0 for wink, bit 1 for double blink, bit 2 for close your eyes, and bit 3 for jump.

pub fn actions(n: u8) -> Vec<&'static str> {
    let mut result = Vec::new();
    for (i, &action) in ACTIONS.iter().enumerate() {
        if n & (1 << i) != 0 {
            result.push(action);
        }
    }
    if n & (1 << 4) != 0 {
        result.reverse();
    }
    result
}
