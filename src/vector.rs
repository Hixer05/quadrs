use std::ops::{Add, Deref, Mul};

pub trait Vectorial: Sized + Add<Output = Self> + Mul<f64, Output = Self> + Clone + Copy {
    fn within(&self, _: (Self, Self)) -> bool; // REVIEW should this be part of the interface?
}

#[derive(Clone, Copy, Debug)]
pub struct DefaultVector<const N: usize>(pub [f64; N]);

impl<const N: usize> Deref for DefaultVector<N> {
    type Target = [f64; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Add for DefaultVector<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self[i] + rhs[i]))
    }
}

impl<const N: usize> Mul<f64> for DefaultVector<N> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(std::array::from_fn(|i| self[i] * rhs))
    }
}

impl<const N: usize> Vectorial for DefaultVector<N> {
    fn within(&self, area: (Self, Self)) -> bool {
        for i in 0..N {
            if !(area.0 .0[i].min(area.1 .0[i]) <= self.0[i]
                && self.0[i] <= area.0 .0[i].max(area.1 .0[i]))
            {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_vector_impl() {
    let p = DefaultVector::<2>([1.0, 2.0]);
    assert_eq!(*(p + p), *(p * 2.0));
}
