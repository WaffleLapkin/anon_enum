use std::{
    future::Future,
    task::{Context, Poll},
    pin::Pin,
    error::Error,
    fmt::{self, Display, Formatter}
};

use crate::{Either, Never};

impl<L, R> Iterator for Either<L, R>
    where
        L: Iterator,
        R: Iterator<Item = L::Item>,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(left) => left.next(),
            Either::Right(right) => right.next(),
        }
    }
}

// Never can't be iterator so we need to special-case that
impl<L> Iterator for Either<L, Never>
    where
        L: Iterator,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Either::Left(left) = self;
        left.next()
    }
}

impl<L, R> Future for Either<L, R>
where
    L: Future,
    R: Future<Output = L::Output>,
{
    type Output = L::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Copy-pasted from futures:
        // https://docs.rs/futures-util/0.3.4/src/futures_util/future/either.rs.html#52-67
        unsafe {
            match self.get_unchecked_mut() {
                Either::Left(x) => Pin::new_unchecked(x).poll(cx),
                Either::Right(x) => Pin::new_unchecked(x).poll(cx),
            }
        }
    }
}

impl<L> Future for Either<L, Never>
where
    L: Future,
{
    type Output = L::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            let Either::Left(x) = self.get_unchecked_mut();
            Pin::new_unchecked(x).poll(cx)
        }
    }
}

impl<L, R> Display for Either<L, R>
where
    L: Display,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Either::Left(left) => left.fmt(f),
            Either::Right(right) => right.fmt(f),
        }
    }
}

impl Display for Never {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl<L, R> Error for Either<L, R>
    where
        L: Error,
        R: Error,
{}

impl Error for Never {}
