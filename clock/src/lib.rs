use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock { h: i32, m:  i32}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let  mut total: i32 =(hours*60 +minutes) % (60*24);
        if total < 0 {
            total+= 24*60;
        }
        Self { h: total/60 , m: total%60 }
       
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total: i32 =(self.h*60 + self.m + minutes) % (24*60);
          if total < 0 {
            total+= 24*60;
        }
        Self {h: total/60 , m: total %60 }
        //todo!("Add {minutes} minutes to existing Clock time");
    }
    
}

  impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.h, self.m)
    }
}
