use super::super::parse::*;
use std::cmp::Ordering;

pub fn sort_by_line_pos<T>(l: &T, r: &T) -> Ordering
where
    T: LinePosition,
{
    if l.line_position() == r.line_position() {
        Ordering::Equal
    } else if l.line_position() < r.line_position() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
