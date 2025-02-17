use std::ops::Deref;

pub(crate) enum MaybeOwned<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<T> Deref for MaybeOwned<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> AsRef<T> for MaybeOwned<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            MaybeOwned::Borrowed(b) => b,
            MaybeOwned::Owned(o) => o,
        }
    }
}

impl<T> Clone for MaybeOwned<'_, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            MaybeOwned::Borrowed(b) => MaybeOwned::Borrowed(b),
            MaybeOwned::Owned(o) => MaybeOwned::Owned(o.clone()),
        }
    }
}
