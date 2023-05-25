use super::super::parse::*;
use std::cmp::Ordering;

pub fn sort_by_line_pos_and_name<T>(l: &T, r: &T) -> Ordering
where
    T: LinePosition + NameString,
{
    if l.line_position() == r.line_position() {
        l.name_string().cmp(&r.name_string())
    } else if l.line_position() < r.line_position() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
