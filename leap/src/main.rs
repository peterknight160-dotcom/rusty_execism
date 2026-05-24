use leap::is_leap_year;
fn main ()
{
 for year in 900..=950 {
    let y = year* 2;    
     if is_leap_year(y) {
         println!("{} is a leap year", y);
     } else {
         println!("{} is not a leap year", y);
     }
 }
}