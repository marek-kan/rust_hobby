use crate::prelude::get_random_normal;

use super::omp::*;
use super::scores::*;
use super::*;

fn make_selector_data(n: usize) -> (Array2<f64>, Array2<f64>, Array2<f64>) {
    let z1 = get_random_normal(n);
    let z2 = get_random_normal(n);

    let x1 = &z1 + 0.05 * get_random_normal(n);
    let x2 = 1.1 * &z1 + 0.05 * get_random_normal(n);
    let x3 = -0.75 * &z1 + 0.05 * get_random_normal(n);
    let x4 = &z2 + 0.05 * get_random_normal(n);
    let x5 = get_random_normal(n);

    let logits = 2.0 * &z1 + 1.5 * &z2 + 0.1;

    let x = ndarray::concatenate(
        Axis(1),
        &[x1.view(), x2.view(), x3.view(), x4.view(), x5.view()],
    )
    .unwrap();

    (x, logits, z2)
}

#[test]
fn test_selector_squared_partial_correlation() {
    let n = 5000;
    let (x, logits, _) = make_selector_data(n);

    let selector = OrthogonalSelector::new(
        vec![0],
        ScoreType::SquaredPartialCorrelation,
        0.5,
        true,
        Some(2),
        None,
    );

    let (mut selected, _scores) = selector.fit(x.view(), logits.view(), None).unwrap();
    selected.sort();

    assert_eq!(selected, vec![0, 3]);
}

#[test]
fn test_selector_residual_variance_ratio() {
    let n = 5000;
    let (x, logits, _) = make_selector_data(n);

    let selector = OrthogonalSelector::new(
        vec![0],
        ScoreType::ResidualVarianceRatio,
        0.05,
        true,
        None,
        None,
    );

    let (mut selected, _scores) = selector.fit(x.view(), logits.view(), None).unwrap();
    selected.sort();

    assert_eq!(selected, vec![0, 3, 4]);
}

#[test]
fn test_selector_logit_gradient() {
    use ndarray_rand::rand;
    use ndarray_rand::rand_distr::{Bernoulli, Distribution};

    let mut rnd_seed = rand::rng();
    let n = 5000;
    let (x, logits, _) = make_selector_data(n);

    let p = 1.0 / (1.0 + (-1.0 * &logits).exp());
    let y_classif = p.mapv(|prob| {
        let bernoulli = Bernoulli::new(prob).unwrap();
        if bernoulli.sample(&mut rnd_seed) {
            1.0
        } else {
            0.0
        }
    });

    let selector = OrthogonalSelector::new(
        vec![0, 3],
        ScoreType::LogitGradient,
        0.05,
        true,
        Some(2),
        None,
    );

    let (mut selected, _scores) = selector.fit(x.view(), y_classif.view(), None).unwrap();
    selected.sort();

    assert_eq!(selected, vec![0, 3]);
}
