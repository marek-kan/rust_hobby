use crate::prelude::get_random_normal;

use super::omp::*;
use super::scores::*;
use super::*;

use ndarray_rand::rand_distr::{Bernoulli, Distribution};

fn make_selector_data(n: usize) -> (Array2<f32>, Array2<f32>, Array2<f32>) {
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
    let mut rnd_seed = rand::rng();
    let n = 5000;
    let (x, logits, _) = make_selector_data(n);

    let p = 1.0 / (1.0 + (-1.0 * &logits).exp());
    let y_classif = p.mapv(|prob| {
        let bernoulli = Bernoulli::new(prob.into()).unwrap();
        if bernoulli.sample(&mut rnd_seed) {
            1.0
        } else {
            0.0
        }
    });

    let selector =
        OrthogonalSelector::new(vec![0], ScoreType::LogitGradient, 0.05, true, Some(2), None);

    let (mut selected, _scores) = selector.fit(x.view(), y_classif.view(), None).unwrap();
    selected.sort();

    assert_eq!(selected, vec![0, 3]);
}

#[test]
fn test_selector_logit_gradient_multiclass() {
    let mut rng = rand::rng();
    let n = 5000;

    let (x, logits_bin, _) = make_selector_data(n);
    let x5 = x.column(4).to_owned().insert_axis(Axis(1));

    let logits_cls2 = 5.0 * x5 - 2.0;

    let mut y_multiclass = Array2::zeros((n, 1));

    for i in 0..n {
        let p_bin = 1.0 / (1.0 + (-logits_bin[[i, 0]]).exp());
        let p_cls2 = 1.0 / (1.0 + (-logits_cls2[[i, 0]]).exp());

        // High probability for class 2 if x5 is large
        if Bernoulli::new(p_cls2.clamp(0.001, 0.999).into())
            .unwrap()
            .sample(&mut rng)
        {
            y_multiclass[[i, 0]] = 2.0;
        } else if Bernoulli::new(p_bin.clamp(0.001, 0.999).into())
            .unwrap()
            .sample(&mut rng)
        {
            y_multiclass[[i, 0]] = 1.0;
        } else {
            y_multiclass[[i, 0]] = 0.0;
        }
    }

    let selector =
        OrthogonalSelector::new(vec![0], ScoreType::LogitGradient, 0.05, true, Some(3), None);

    let (mut selected, _scores) = selector.fit(x.view(), y_multiclass.view(), None).unwrap();
    selected.sort();

    assert_eq!(selected, vec![0, 3, 4]);
}
