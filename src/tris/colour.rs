use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Colour {
    Empty,
    Value(u16),
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colour::Empty => write!(f, "empty"),
            Colour::Value(v) => write!(f, "#{0}", v),
        }
    }
}
