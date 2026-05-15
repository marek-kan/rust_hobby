use ndarray_stats::QuantileExt;

use super::estimator::*;
use super::*;
use crate::prelude::get_random_normal;

#[test]
fn test_scaler() {
    let arr: Array2<f32> = array![[0.0], [1.0], [0.5], [5.0], [3.0]];
    let expected_std = arr.std_axis(Axis(0), 0.0);
    let expected_mu = arr.mean_axis(Axis(0)).unwrap();

    let mut scaler = StandardScaler::default();

    let transformed = scaler.fit_transform(&arr).unwrap();

    let mu = scaler.mu.unwrap();
    let stds = scaler.std.unwrap();

    assert!((mu.column(0)[0] - expected_mu[0]).abs() <= EPS);
    assert!((stds.column(0)[0] - expected_std[0]).abs() <= EPS);

    assert!(&transformed.mean().unwrap().abs() < &EPS);
    assert!((&transformed.std(0.0) - 1.0).abs() < EPS);
}

#[test]
fn test_fitpredict_standardized_runs() {
    let features: Array2<f32> = array![[0.0], [1.0], [0.5], [5.0], [3.0]];
    let y: Array2<f32> = array![[0.], [0.], [0.], [1.], [1.]];
    let mut scaler = StandardScaler::new();

    let x = scaler.fit_transform(&features).unwrap();

    let mut model = LogisticRegression::default();
    let _ = model.fit(&x, &y, &None);

    assert!(model.is_fitted());

    let pred_proba = model.predict_proba(&x).unwrap();
    let pred = model.predict(&x).unwrap();

    assert_eq!(pred.nrows(), y.nrows());

    for p in pred_proba.iter() {
        assert!(*p > 0.0 && *p < 1.0);
    }

    for p in pred.iter() {
        assert!(*p == 1.0 || *p == 0.0);
    }
}

#[test]
fn test_coefficients_change() {
    let features: Array2<f32> = array![[0.0], [1.0], [0.5], [5.0], [3.0]];
    let y: Array2<f32> = array![[0.], [0.], [0.], [1.], [1.]];
    let mut scaler = StandardScaler::new();

    let x = scaler.fit_transform(&features).unwrap();

    let mut model = LogisticRegression::default();
    let _ = model.fit(&x, &y, &None);

    assert!(model.is_fitted());

    let coeff = model.coeff.unwrap();

    let norm: f32 = coeff.iter().map(|v| v.abs()).sum();
    let intercept = model.intercept.unwrap().abs();

    assert!(intercept > 0.0);
    assert!(norm > 0.0);
}

#[test]
fn test_multitarget_estimator() {
    let n = 500;

    let mut features: Array2<f32> = ndarray::concatenate(
        Axis(1),
        &[get_random_normal(n).view(), get_random_normal(n).view()],
    )
    .unwrap();

    let x3: Array2<f32> = (&features.column(0) * &features.column(1))
        .insert_axis(Axis(1))
        .to_owned();

    features = ndarray::concatenate(Axis(1), &[features.view(), x3.view()]).unwrap();

    let y: Array2<f32> = features
        .map_axis(Axis(1), |a| a.argmax().unwrap() as f32)
        .insert_axis(Axis(1));
    let mut scaler = StandardScaler::new();

    let x = scaler.fit_transform(&features).unwrap();

    let mut model = OneVsAll::default();
    let _ = model.fit(&x, &y, &None);

    let predictions_proba = model.predict_proba(&x).unwrap();
    let predictions_class = model.predict(&x).unwrap();

    assert_eq!(predictions_proba.nrows(), 500);
    assert_eq!(predictions_proba.ncols(), 3);
    assert!(*predictions_proba.max().unwrap() <= 1.0);

    assert_eq!(predictions_class.nrows(), 500);
    assert_eq!(predictions_class.ncols(), 1);

    assert!(model.is_fitted());

    let coeff: Vec<Array1<f32>> = model
        .estimators
        .iter()
        .map(|(_, est)| est.coeff.clone().unwrap())
        .collect();

    println!("{:?}", &coeff);

    let norm: f32 = coeff.iter().fold(0.0, |acc, v| acc + v.abs().sum());
    let intercept = model
        .estimators
        .iter()
        .fold(0.0, |acc, (_, est)| acc + est.intercept.unwrap().abs());

    assert!(intercept > 0.0);
    assert!(norm > 0.0);
}
