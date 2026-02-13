use std::fmt;
const DAY_IN_MINUTES:i32=24*60;
pub struct Clock {
    clock_hours: i32,
    clock_minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes=(hours*60)+minutes;
        let normalized_minutes=total_minutes.rem_euclid(DAY_IN_MINUTES);
        Clock{
            clock_hours:normalized_minutes/60,
            clock_minutes:normalized_minutes%60,
        }

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.clock_hours,self.clock_minutes+minutes)
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.clock_hours, self.clock_minutes)
    }
}
impl PartialEq for Clock{
    fn eq(&self, other:&Clock)->bool{
        self.clock_hours==other.clock_hours && self.clock_minutes==other.clock_minutes
    }
}
impl fmt::Debug for Clock{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        f.debug_struct("Clock")
        .field("clock_hours",&self.clock_hours)
        .field("clock_minutes",&self.clock_minutes)
        .finish()
    }
}
