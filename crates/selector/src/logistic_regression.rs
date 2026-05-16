pub(crate) mod estimator;
pub(crate) mod models;
use crate::prelude::*;
use estimator::*;
use models::*;
use ndarray_stats::QuantileExt;

#[derive(Clone, Copy)]
pub struct LogisticRegressionParams {
    pub max_iter: usize,
    pub alpha: f32,
    pub learning_rate: f32,
    pub r_tol: f32,
}

impl LogisticRegressionParams {
    pub fn new(max_iter: usize, alpha: f32, learning_rate: f32, r_tol: f32) -> Self {
        Self {
            max_iter,
            alpha,
            learning_rate,
            r_tol,
        }
    }
}

impl Default for LogisticRegressionParams {
    fn default() -> Self {
        LogisticRegressionParams {
            max_iter: 500,
            alpha: 1.0,
            learning_rate: 0.05,
            r_tol: 1e-4,
        }
    }
}

#[cfg(test)]
mod test;
