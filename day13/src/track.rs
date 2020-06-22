#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Track {
    Intersection,
    Horizontal,
    Vertical,
    DiagonalLeft,
    DiagonalRight,
    Empty,
}

impl Track {
    pub fn to_char(&self) -> char {
        match self {
            Track::Intersection => '+',
            Track::Horizontal => '-',
            Track::Vertical => '|',
            Track::DiagonalRight => '/',
            Track::DiagonalLeft => '\\',
            Track::Empty => ' ',
        }
    }
}
