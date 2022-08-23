use std::ops::Range;

pub struct Error {
    pub message: String,
    pub location: Range<usize>,
}
