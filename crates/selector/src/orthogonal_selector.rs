use crate::prelude::*;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::collections::HashMap;

use crate::logistic_regression::estimator::*;

pub mod omp;
mod scores;
#[cfg(test)]
mod test;
