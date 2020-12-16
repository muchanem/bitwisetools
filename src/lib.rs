#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode_encode() {

        let reference = vec!["thing1", "thing2", "thing3"];
        assert_eq!(5, encode_bitwise(reference.clone(), decode_bitwise(reference, 5)));
    }
    #[test]
    fn encode() {
        let reference = vec!["thing1", "thing2", "thing3"];
        let actual = vec!["thing1", "thing3"];
        assert_eq!(5, encode_bitwise(reference, actual))
    }
}

pub fn decode_bitwise<T>(i: T, n: u32) -> T
where
    T: IntoIterator + std::iter::FromIterator<<T as std::iter::IntoIterator>::Item> + std::fmt::Debug,
{
    i.into_iter().enumerate().filter(|(i, _x)| (1 << i) == (n & (1 << i))).map(|(_i, x)| x).collect::<T>()
}

// probably shouldn't pass T with duplicate T::Item
pub fn encode_bitwise<T>(r: T, w: T) -> u32
where
    T: IntoIterator,
    <T as std::iter::IntoIterator>::IntoIter: std::clone::Clone,
    <T as std::iter::IntoIterator>::Item: std::cmp::PartialEq
{
    let ri = r.into_iter();
    let mut n = 0;
    w.into_iter().for_each(|x| n += 1 << ri.clone().position(|v| v == x).unwrap_or(0));
    n as u32
}
 