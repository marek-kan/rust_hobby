use crate::prelude::*;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::logistic_regression::estimator::*;

pub mod omp;
mod scores;

pub use omp::{LogisticRegressionParams, OrthogonalError, OrthogonalSelector};
pub use scores::ScoreType;

#[cfg(test)]
mod test;
