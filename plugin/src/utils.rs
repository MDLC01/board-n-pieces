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
