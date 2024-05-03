// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

pub trait MapInto<T> {
    fn map_into(self) -> T;
}

impl<T, U, E> MapInto<Result<U, E>> for Result<T, E>
where
    T: Into<U>,
{
    #[inline]
    fn map_into(self) -> Result<U, E> {
        self.map(|value| value.into())
    }
}

impl<T, U> MapInto<Option<U>> for Option<T>
where
    T: Into<U>,
{
    #[inline]
    fn map_into(self) -> Option<U> {
        self.map(|value| value.into())
    }
}

// impl<TIt, UIt> MapInto<UIt> for TIt
// where
//     TIt: Iterator<Item = T>,
//     UIt: Iterator<Item = U>,
// {
//     #[inline]
//     fn map_into(self) -> UIt {
//         self.map(Into::into)
//     }
// }

// todo : skill issues

// pub trait AllInto<T> {
//     fn all_into(self) -> T;
// }
//
// impl<ItT, T, U, Z> AllInto<Z> for ItT
// where
//     ItT: IntoIterator<Item = T>,
//     T: Into<U>,
//     Z: FromIterator<U>,
// {
//     #[inline]
//     fn all_into(self) -> Vec<U> {
//         self.into_iter().map(|t| t.into()).collect()
//     }
// }

// pub trait MapAllInto<T> {
//     fn map_all_into(self) -> T;
// }
//
// impl<T, TIt, UIt, E> MapAllInto<std::result::Result<UIt, E>> for std::result::Result<TIt, E>
// where
//     TIt: IntoIterator<Item = T>,
//     UIt: FromIterator,
// {
//     #[inline]
//     fn map_all_into(self) -> std::result::Result<UIt, E> {
//         self.map(|value| value.into_iter().map(|t| t.into()).collect())
//     }
// }
