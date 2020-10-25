use std::fmt::{Display, Formatter, Result};

pub(crate) struct WrappedInt {
    pub i: i32
}

impl Display for WrappedInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.i)
    }
}
