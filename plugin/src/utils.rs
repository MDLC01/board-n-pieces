/// Similar to [`FromStr`](std::str::FromStr).
pub trait FromChar: Sized {
    type Err;

    fn from_char(c: char) -> Result<Self, Self::Err>;
}

pub trait CharExt {
    fn parse<T: FromChar>(self) -> Result<T, T::Err>;
}

impl CharExt for char {
    fn parse<T: FromChar>(self) -> Result<T, T::Err> {
        T::from_char(self)
    }
}

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

pub trait StrExt {
    /// If this string slice is not empty, returns its last character and the corresponding prefix.
    /// Otherwise, returns `None`.
    fn split_last_char(&self) -> Option<(&str, char)>;
}

impl StrExt for str {
    fn split_last_char(&self) -> Option<(&str, char)> {
        self.chars()
            .last()
            .map(|c| (&self[..self.len() - c.len_utf8()], c))
    }
}

/// A trait for finite types.
///
/// Notably, defines the [`values`](Self::values) function that returns all the values of the type.
pub trait Finite: Sized {
    /// Returns all the values of this type.
    fn values() -> impl IntoIterator<Item = Self>;

    /// Returns an iterator over all the values of the type
    fn iter() -> impl Iterator<Item = Self> {
        Self::values().into_iter()
    }
}

/// A trait for types for which each value has a canonical name.
pub trait Name {
    fn name(&self) -> String;
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
