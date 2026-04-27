use super::estimator::*;
use super::*;

#[test]
fn test_scaler() {
    let arr: Array2<f64> = array![[0.0], [1.0], [0.5], [5.0], [3.0]];
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
    let features: Array2<f64> = array![[0.0], [1.0], [0.5], [5.0], [3.0]];
    let y: Array2<f64> = array![[0.], [0.], [0.], [1.], [1.]];
    let mut scaler = StandardScaler::new();

    let x = scaler.fit_transform(&features).unwrap();

    let mut model = LogisticRegression::default();
    let _ = model.fit(&x, &y, None);

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
    let features: Array2<f64> = array![[0.0], [1.0], [0.5], [5.0], [3.0]];
    let y: Array2<f64> = array![[0.], [0.], [0.], [1.], [1.]];
    let mut scaler = StandardScaler::new();

    let x = scaler.fit_transform(&features).unwrap();

    let mut model = LogisticRegression::default();
    let _ = model.fit(&x, &y, None);

    let coeff = model.coeff.unwrap();

    let norm: f64 = coeff.iter().map(|v| v.abs()).sum();
    let intercept = model.intercept.unwrap().abs();

    assert!(intercept > 0.0);
    assert!(norm > 0.0);
}
