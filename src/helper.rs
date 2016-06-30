use rand::{thread_rng, Rng, Rand};
use rand::distributions::range::SampleRange;

#[inline]
pub fn gen_range<T: PartialOrd + SampleRange>(start: T, end: T) -> T {
    thread_rng().gen_range(start, end)
}

#[inline]
pub fn random<T: Rand>() -> T {
    thread_rng().gen()
}

#[inline]
pub fn shuffle<T: Clone>(arr: &[T]) -> Vec<T> {
    let mut v = arr.to_vec();
    thread_rng().shuffle(&mut *v);
    v
}