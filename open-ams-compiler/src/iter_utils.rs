pub(crate) fn chunked<I, F, T>(iter: I, func: F) -> impl Iterator<Item = T>
where
    I: IntoIterator,
    I::Item: Clone,
    F: Fn(I::Item, I::Item) -> T,
{
    let mut iter = iter.into_iter();
    std::iter::from_fn(move || {
        if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
            Some(func(a, b))
        } else {
            None
        }
    })
}
