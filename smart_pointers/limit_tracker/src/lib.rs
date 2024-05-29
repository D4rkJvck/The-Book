pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        match self.value as f64 / self.max as f64 {
            result if result > 1.0 => self
                .messenger
                .send("ERROR: Unfortunately, You are over your quota !"),

            result if result >= 0.9 => self
                .messenger
                .send("URGENT: You've used up over 90% of your quota !"),

            result if result >= 0.75 => self
                .messenger
                .send("WARNING: You've used up over 75% of your quota !"),

            _ => todo!(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////
