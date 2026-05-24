// Can't use format!() or println!() macros, as they require string literals, which makes the code non-generic.

static LINE1: &str = "For want of a FIRST the SECOND was lost.";
static LINE2: &str = "And all for the want of a FIRST.";

pub fn build_proverb(list: &[&str]) -> String {

    let mut result = String::new();
    if list.is_empty() {
        return result;
    }
  
    for i in 0..list.len() - 1 {
        let line = LINE1.replace("FIRST", list[i]).replace("SECOND", list[i + 1]);
        result.push_str(&line);
        result.push('\n');
    }   
    let line = LINE2.replace("FIRST", list[0]);
    result.push_str(&line   );
    result


}
