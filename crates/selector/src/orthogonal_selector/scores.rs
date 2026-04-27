use super::omp::*;
use super::*;

pub enum ScoreType {
    ResidualVarianceRatio,
    SquaredPartialCorrelation,
    LogitGradient,
}

pub fn residual_variance_ratio(x: &Array2<f64>, r: &Array2<f64>, w: &Array2<f64>) -> f64 {
    let denom = (w * x * x).sum().max(EPS);

    (w * r * r).sum() / denom
}

pub fn squared_partial_correlation(
    x_orth: &Array2<f64>,
    y: &Array2<f64>,
    q: &Array2<f64>,
    w: &Array2<f64>,
) -> f64 {
    let r_y = orthogonalize(q, y, w);
    let denom_x = (w * x_orth * x_orth).sum();
    let denom_y = (w * &r_y * &r_y).sum();

    let denom = (denom_x * denom_y).max(EPS);
    let numer = (w * x_orth * &r_y).sum().powi(2);

    println!(
        "Denom X: {}, Denom Y: {}, Numer: {}, Denom: {}",
        denom_x, denom_y, numer, denom
    );

    numer / denom
}

pub fn delta_log_loss(
    logistic_regression_params: &LogisticRegressionParams,
    data: &Array2<f64>,
    x_orth: &Array2<f64>,
    y: &Array2<f64>,
    w: &Array2<f64>,
) -> Result<f64, FitError> {
    let estimator = LogisticRegression::new(
        logistic_regression_params.max_iter,
        logistic_regression_params.alpha,
        logistic_regression_params.learning_rate,
        logistic_regression_params.r_tol,
    );

    if data.ncols() < 1 {
        let mut model = estimator.clone();

        let x_fit = StandardScaler::new().fit_transform(x_orth)?;

        model.fit(&x_fit, y, Some(w.clone()))?;

        let logits = model.decision_boundary(&x_fit)?;

        return model.loss(&logits, y, w);
    }

    let mut model_base = estimator.clone();
    let x_fit_base = StandardScaler::new().fit_transform(data)?;

    model_base.fit(&x_fit_base, y, Some(w.clone()))?;

    let loss_base = model_base.loss(&model_base.decision_boundary(&x_fit_base)?, y, w)?;

    let mut data_new = data.clone();
    data_new
        .push_column(x_orth.column(0))
        .expect("Unexpected shape error during `delta_log_loss`");

    let x_fit_new = StandardScaler::new().fit_transform(&data_new)?;
    let mut model_new = estimator.clone();

    model_new.fit(&x_fit_new, y, Some(w.clone()))?;

    let loss_new = model_new.loss(&model_new.decision_boundary(&x_fit_new)?, y, w)?;

    Ok(loss_base - loss_new)
}
