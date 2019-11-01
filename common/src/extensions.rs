pub trait AbsDiff<T = Self> {
    fn abs_diff(self, other: T) -> Self;
}

pub trait IteratorExt: Iterator {
    fn uniq_min_by_key<B: Ord>(self, f: impl FnMut(&Self::Item) -> B) -> Option<Self::Item>;
}

pub trait OptionMerge<T> {
    fn merge<S>(self, b: Option<S>) -> Option<(T, S)>;
}

pub trait ResultMerge<T, E> {
    fn merge<S>(self, b: Result<S, E>) -> Result<(T, S), E>;
}

pub fn cart_product<A: Copy, B: Copy>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |a| b.clone().map(move |b| (a, b)))
}

impl<T> OptionMerge<T> for Option<T> {
    fn merge<S>(self, b: Option<S>) -> Option<(T, S)> {
        self.and_then(|a| b.map(|b| (a, b)))
    }
}

impl<T, E> ResultMerge<T, E> for Result<T, E> {
    fn merge<S>(self, b: Result<S, E>) -> Result<(T, S), E> {
        self.and_then(|a| b.map(|b| (a, b)))
    }
}

impl AbsDiff for usize {
    fn abs_diff(self, other: usize) -> usize {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

impl AbsDiff for u16 {
    fn abs_diff(self, other: u16) -> u16 {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

impl AbsDiff<i16> for u16 {
    fn abs_diff(self, other: i16) -> u16 {
        (other - self as i16).abs() as u16
    }
}

impl<I: Iterator> IteratorExt for I {
    fn uniq_min_by_key<B: Ord>(mut self, mut f: impl FnMut(&Self::Item) -> B) -> Option<I::Item> {
        let first = self.next()?;
        let value = f(&first);
        let first = (first, value, true);

        let (min_e, _, uniq) = self.fold(first, |(min_e, min_v, uniq), e| {
            let v = f(&e);
            if v < min_v {
                (e, v, true)
            } else {
                let uniq = uniq && min_v != v;
                (min_e, min_v, uniq)
            }
        });
        if !uniq {
            None
        } else {
            Some(min_e)
        }
    }
}
