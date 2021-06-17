use std::str::FromStr;

#[derive(Debug)]
pub enum ExtChar<T> {
    Char(T),
    Optional(T),
    BoundedRepeat(T, std::ops::Range<usize>),
    Repeat(T),
}

impl<T: PartialEq> PartialEq for ExtChar<T> {
    fn eq(&self, oth: &Self) -> bool {
        use ExtChar::*;
        match (self, oth) {
            (Char(s), Char(o)) => s == o,
            (Optional(s), Optional(o)) => s == o,
            (BoundedRepeat(s, sr), BoundedRepeat(o, or)) => s == o && sr == or,
            (Repeat(s), Repeat(o)) => s == o,
            _ => false,
        }
    }
}

impl<T> FromStr for ExtChar<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ExtChar::*;
        let cnv = T::from_str(s)?;
        let mut itr = s.chars();
        if let Some(c) = itr.next() {
            let c = T::from(c);
            
    }
}

#[cfg(test)]
mod test {
    use super::ExtChar;
    use std::str::FromStr;
    #[test]
    fn from_test() {
        let a = "a";
        let s_a = String::from(a);
        assert_eq!(ExtChar::Char('a'), ExtChar::from_str(a).unwrap());
        assert_eq!(ExtChar::Char('a'), ExtChar::from_str(&s_a).unwrap());
    }
}
