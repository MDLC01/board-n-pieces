pub trait SliceExt<T> {
    fn split_on<'a>(&'a self, value: T) -> impl Iterator<Item = &'a [T]>
    where
        T: PartialEq + 'a;
}

impl<T> SliceExt<T> for [T] {
    fn split_on<'a>(&'a self, value: T) -> impl Iterator<Item = &'a [T]>
    where
        T: PartialEq + 'a,
    {
        self.split(move |e| *e == value)
    }
}

pub trait OptionExt<T> {
    #[allow(clippy::wrong_self_convention)]
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> OptionExt<T> for Option<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            None => true,
            Some(x) => f(x),
        }
    }
}

/// Iterates over all pairs of elements of two iterators.
///
/// The order of iteration is best described by the following example:
/// ```
/// cartesian_product(0..3, 0..2).map(|x| println!("{:?}", x));
/// // Prints:
/// // (0, 0)
/// // (0, 1)
/// // (1, 0)
/// // (1, 1)
/// // (2, 0)
/// // (2, 1)
/// ```
pub fn cartesian_product<I, J>(lhs: I, rhs: J) -> impl Iterator<Item = (I::Item, J::Item)>
where
    I: IntoIterator,
    I::Item: Clone,
    J: IntoIterator,
    J::IntoIter: Clone,
{
    let rhs = rhs.into_iter();
    lhs.into_iter()
        .flat_map(move |l| rhs.clone().map(move |r| (l.clone(), r)))
}
