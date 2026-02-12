use ndarray::prelude::*;
use ndarray_rand::{RandomExt, rand_distr::Normal};

pub fn get_random_normal(n: usize) -> Array2<f64> {
    Array::random((n, 1), Normal::new(0.0, 1.0).unwrap())
}
