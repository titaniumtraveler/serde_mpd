use std::fmt::{Debug, Formatter};

pub struct SliceDebug<T>(pub T);

impl Debug for SliceDebug<&[u8]> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(self.0) {
            Ok(str) => write!(f, "b{:?}", str),
            Err(_) => self.0.fmt(f),
        }
    }
}
impl Debug for SliceDebug<&[&[u8]]> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(|slice| SliceDebug(*slice)))
            .finish()
    }
}

impl<T: PartialEq> PartialEq<SliceDebug<T>> for SliceDebug<T> {
    fn eq(&self, other: &SliceDebug<T>) -> bool {
        self.0.eq(&other.0)
    }
}
