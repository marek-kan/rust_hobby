use ndarray::{Data, ShapeError, linalg::Dot, prelude::*};
use ndarray_rand::{RandomExt, rand_distr::Normal};
use std::collections::HashMap;
use thiserror::Error;

const EPS: f64 = 1e-12;

pub enum ScoreType {
    ResidualVarianceRatio,
    SquaredPartialCorrelation,
    LogitGradient,
}
#[derive(Error, Debug)]
pub enum OrthogonalError {
    #[error("Fixed feature not orthogonal: {0} to the set: {1:?}")]
    FixedFeatureNotOrthogonal(usize, Vec<usize>),

    #[error[transparent]]
    QShapeError(#[from] ShapeError),
}

pub struct LogisticRegressionParams {
    max_iter: usize,
    alpha: f64,
    learning_rate: f64,
    r_tol: f64,
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

pub struct OrthogonalSelector {
    fixed_feature_indices: Vec<usize>,
    score_type: ScoreType,
    min_score: f64,
    center_featues: bool,
    logistic_regression_params: LogisticRegressionParams,
}

impl OrthogonalSelector {
    pub fn new(
        fixed_feature_indices: Vec<usize>,
        score_type: ScoreType,
        min_score: f64,
        center_featues: bool,
        logistic_regression_params: Option<LogisticRegressionParams>,
    ) -> Self {
        let log_reg_params = logistic_regression_params.unwrap_or_default();

        OrthogonalSelector {
            fixed_feature_indices,
            score_type,
            min_score,
            center_featues,
            logistic_regression_params: log_reg_params,
        }
    }

    fn calculate_score(
        &self,
        x: &Array2<f64>,
        x_orth: &Array2<f64>,
        q: &Array2<f64>,
        y: &Array2<f64>,
        sw: &Array2<f64>,
    ) -> f64 {
        match self.score_type {
            ScoreType::ResidualVarianceRatio => residual_variance_ratio(&x, &x_orth, &sw),
            ScoreType::SquaredPartialCorrelation => {
                squared_partial_correlation(&x_orth, &y, &q, &sw)
            }
            ScoreType::LogitGradient => {
                delta_log_loss(&self.logistic_regression_params, &q, &x_orth, &y, &sw)
                    .expect("Unexpected error during `delta_log_loss`")
            }
        }
    }

    fn weighted_center(&self, data: &Array2<f64>, sample_weights: &Array2<f64>) -> Array2<f64> {
        let weight_sum = sample_weights.sum();
        let mut means = data.t().dot(sample_weights) / weight_sum;
        means.reverse_axes();

        data - means
    }

    pub fn fit(
        &self,
        data: &mut Array2<f64>,
        y: &Array2<f64>,
        sample_weights: Option<Array2<f64>>,
    ) -> Result<(Vec<usize>, HashMap<usize, f64>), OrthogonalError> {
        let n = data.nrows();

        let sw = match sample_weights {
            Some(sw) => sw,
            None => Array::ones((n, 1)),
        };

        let mut q: Array2<f64> = Array2::zeros((n, 0));
        let mut selected: Vec<usize> = vec![];
        let mut scores: HashMap<usize, f64> = HashMap::new();
        let mut explore_feature_indices: Vec<usize> = (0..data.ncols()).collect();
        println!("{:?}", explore_feature_indices);

        if self.center_featues {
            *data = self.weighted_center(data, &sw)
        }

        for i in &self.fixed_feature_indices {
            let idx = i.to_owned();
            let pop_idx = explore_feature_indices
                .iter()
                .position(|f_idx| f_idx == i)
                .expect("couldn't find index of fixed feature");
            explore_feature_indices.remove(pop_idx); // remove fixed features from upcomming search

            let x = data.slice(s![.., idx..idx + 1]).to_owned();

            let x_orth = orthogonalize(&q, &x, &sw);
            let x_norm = weighted_norm(&x_orth, &sw);

            let score = self.calculate_score(&x, &x_orth, &q, y, &sw);
            println!("Idx: {} has score: {}", idx, score);

            q.push_column((x_orth / x_norm).column(0))?;
            selected.push(idx);
            scores.insert(idx, score);
        }

        println!("{:?}", explore_feature_indices);

        let mut intermediate_results: HashMap<usize, f64> = HashMap::new();
        while explore_feature_indices.len() > 0 {
            intermediate_results.clear();

            for &idx in &explore_feature_indices {
                let x = data.slice(s![.., idx..idx + 1]).to_owned();
                let x_orth = orthogonalize(&q, &x, &sw);

                let score = self.calculate_score(&x, &x_orth, &q, y, &sw);
                println!("Idx: {} has score: {}", idx, score);

                intermediate_results.insert(idx, score);
            }

            let best_feature = intermediate_results
                .iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap());

            if let Some((&best_feature_idx, &best_feature_score)) = best_feature {
                scores.insert(best_feature_idx, best_feature_score);

                if best_feature_score >= self.min_score {
                    let x = data
                        .slice(s![.., best_feature_idx..best_feature_idx + 1])
                        .to_owned();
                    let x_orth = orthogonalize(&q, &x, &sw);
                    let x_norm = weighted_norm(&x_orth, &sw);

                    q.push_column((x_orth / x_norm).column(0))?;
                    selected.push(best_feature_idx);

                    explore_feature_indices.remove(
                        explore_feature_indices
                            .iter()
                            .position(|x| x == &best_feature_idx)
                            .expect(
                                "Failed to find `best_feature_index` in `explore_feature_indices`",
                            ),
                    );
                } else {
                    // Add scores from last iteration
                    for k in intermediate_results.keys() {
                        if !scores.contains_key(k) {
                            scores.insert(k.clone(), intermediate_results.get(k).unwrap().clone());
                        }
                    }
                    break;
                }
            } else {
                break;
            };
        }

        Ok((selected, scores))
    }
}

pub fn get_random_normal(n: usize) -> Array2<f64> {
    Array::random((n, 1), Normal::new(0.0, 1.0).unwrap())
}

pub fn weighted_inner<T, D>(x: &ArrayBase<T, D>, y: &ArrayBase<T, D>, w: &ArrayBase<T, D>) -> f64
where
    T: Data<Elem = f64>,
    D: Dimension,
{
    let product = w * x * y;
    product.sum()
}

pub fn weighted_norm(x: &Array2<f64>, w: &Array2<f64>) -> f64 {
    let product = (w * x).t().dot(x);
    product.sum().sqrt()
}

pub fn orthogonalize(q: &Array2<f64>, x: &Array2<f64>, w: &Array2<f64>) -> Array2<f64> {
    let mut r = x.clone();

    if q.ncols() != 0 {
        let wr = w * &r;
        let coeff = q.t().dot(&wr);
        r = r - q.dot(&coeff);

        // stability pass
        let wr = w * &r;
        let coeff = q.t().dot(&wr);
        r = r - q.dot(&coeff);
    };

    r
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

#[derive(Default)]
pub struct StandardScaler {
    mu: Option<Array2<f64>>,
    std: Option<Array2<f64>>,
}
impl StandardScaler {
    fn is_fitted(&self) -> bool {
        self.mu.is_some()
    }

    pub fn new() -> Self {
        StandardScaler {
            mu: None,
            std: None,
        }
    }

    pub fn fit(&mut self, x: &Array2<f64>) -> Result<(), FitError> {
        if !self.is_fitted() {
            if let Some(mu) = x.mean_axis(Axis(0)) {
                self.mu = Some(mu.insert_axis(Axis(0)))
            } else {
                return Err(FitError::ErrorDuringFit(
                    "Failed to compute mean in `StandardScaler`".into(),
                ));
            };

            let stds = x.std_axis(Axis(0), 0.).insert_axis(Axis(0));

            if let Some((idx, _)) = stds.iter().enumerate().find(|(_, v)| v.abs() < EPS) {
                return Err(FitError::ConstantFeatureAt(idx));
            }

            self.std = Some(stds);

            Ok(())
        } else {
            Err(FitError::AlreadyFitted)
        }
    }

    pub fn transform(&self, x: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        if self.is_fitted() {
            let mu = self.mu.as_ref().unwrap();
            let s = self.std.as_ref().unwrap();

            Ok((x - mu) / s)
        } else {
            Err(FitError::NotFitted)
        }
    }

    pub fn fit_transform(&mut self, x: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        self.fit(x)?;
        self.transform(x)
    }
}

#[derive(Debug)]
pub enum FitError {
    NotFitted,
    ErrorDuringFit(String),
    ConstantFeatureAt(usize),
    AlreadyFitted,
}

#[derive(Clone)]
pub struct LogisticRegression {
    max_iter: usize,
    alpha: f64,
    learning_rate: f64,
    r_tol: f64,
    pub coeff: Option<Array1<f64>>,
    pub intercept: Option<f64>,
    pub losses: Vec<f64>,
}

impl LogisticRegression {
    pub fn new(max_iter: usize, alpha: f64, learning_rate: f64, r_tol: f64) -> Self {
        LogisticRegression {
            max_iter,
            alpha,
            learning_rate,
            r_tol,
            coeff: None,
            intercept: None,
            losses: vec![],
        }
    }

    pub fn is_fitted(&self) -> bool {
        self.coeff.is_some()
    }

    fn sigmoid(&self, z: &Array2<f64>) -> Array2<f64> {
        z.mapv(|z| {
            if z >= 0.0 {
                1.0 / (1.0 + (-z).exp())
            } else {
                let ez = z.exp();
                ez / (1.0 + ez)
            }
        })
    }

    pub fn decision_boundary(&self, X: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        let coeff = self.coeff.as_ref().ok_or(FitError::NotFitted)?;
        let mut z = X.dot(&coeff.view().insert_axis(Axis(1)));

        if let Some(intercept) = self.intercept {
            // we might not have intercept
            z += intercept;
        }

        Ok(z)
    }

    pub fn predict(&self, X: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        let proba = self.predict_proba(X)?;

        Ok(proba.mapv(|p| if p >= 0.5 { 1.0 } else { 0.0 }))
    }

    pub fn predict_proba(&self, X: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        let z = self.decision_boundary(X)?;

        let pred = self.sigmoid(&z);
        Ok(pred.mapv(|x| x.clamp(EPS, 1.0 - EPS)))
    }

    fn loss(
        &self,
        logits: &Array2<f64>,
        y: &Array2<f64>,
        sample_weights: &Array2<f64>,
    ) -> Result<f64, FitError> {
        let c = self.coeff.as_ref().ok_or(FitError::NotFitted)?;
        let weight_sum = sample_weights.sum();

        // logaddexp(0, z) = ln(1 + exp(z))  (stable form below)
        let log_term = logits.mapv(|z| {
            if z > 0.0 {
                z + (-z).exp().ln_1p()
            } else {
                z.exp().ln_1p()
            }
        });

        let base_loss = (sample_weights * (log_term - y * logits)).sum() / weight_sum;

        let reg_loss = self.alpha / (2.0 * weight_sum) * c.pow2().sum();

        Ok(base_loss + reg_loss)
    }

    fn update(
        &mut self,
        X: &Array2<f64>,
        logits: &Array2<f64>,
        y: &Array2<f64>,
        sample_weights: &Array2<f64>,
    ) -> Result<(), FitError> {
        let pred = self.sigmoid(logits);
        let diff_pred = (&pred - y) * sample_weights;
        let norm = 1.0 / sample_weights.sum();

        // Gradients
        let grad0 = self.learning_rate * norm * diff_pred.sum();
        let grad1 = self.learning_rate * norm * X.t().dot(&diff_pred);
        let decay = 1.0 - self.learning_rate * self.alpha * norm;

        // Update
        let weights = self.coeff.as_mut().unwrap();
        if let Some(intercept) = self.intercept.as_mut() {
            *intercept -= grad0;
        };

        *weights *= decay;
        *weights -= &grad1.column(0);

        Ok(())
    }

    pub fn fit(
        &mut self,
        X: &Array2<f64>,
        y: &Array2<f64>,
        sample_weights: Option<Array2<f64>>,
    ) -> Result<(), FitError> {
        let n = X.nrows();

        let sw = match sample_weights {
            Some(sw) => sw,
            None => Array::ones((n, 1)),
        };

        self.coeff = Some(Array::zeros(X.ncols()));
        self.intercept = Some(0.0);

        let mut last_loss = f64::INFINITY;

        for _ in 0..self.max_iter {
            let logits_hat = self.decision_boundary(X)?;

            let loss = self.loss(&logits_hat, y, &sw)?;
            self.losses.push(loss);

            if (last_loss - loss).abs() / last_loss.abs().max(1.0) <= self.r_tol {
                break;
            } else {
                self.update(X, &logits_hat, y, &sw)?;
                last_loss = loss
            }
        }

        Ok(())
    }
}

impl Default for LogisticRegression {
    fn default() -> Self {
        LogisticRegression {
            max_iter: 100,
            alpha: 1.0,
            learning_rate: 0.1,
            r_tol: 1e-4,
            coeff: None,
            intercept: None,
            losses: vec![],
        }
    }
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

#[cfg(test)]
mod tests {
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
        let (mut x, logits, _) = make_selector_data(n);

        let selector = OrthogonalSelector::new(
            vec![0],
            ScoreType::SquaredPartialCorrelation,
            0.5,
            true,
            None,
        );

        let (selected, _scores) = selector.fit(&mut x, &logits, None).unwrap();

        assert_eq!(selected, vec![0, 3]);
    }

    #[test]
    fn test_selector_residual_variance_ratio() {
        let n = 5000;
        let (mut x, logits, _) = make_selector_data(n);

        let selector =
            OrthogonalSelector::new(vec![0], ScoreType::ResidualVarianceRatio, 0.05, true, None);

        let (selected, _scores) = selector.fit(&mut x, &logits, None).unwrap();

        assert_eq!(selected, vec![0, 3, 4]);
    }

    #[test]
    fn test_selector_logit_gradient() {
        use ndarray_rand::rand;
        use ndarray_rand::rand_distr::{Bernoulli, Distribution};

        let mut rnd_seed = rand::rng();
        let n = 5000;
        let (mut x, logits, _) = make_selector_data(n);

        let p = 1.0 / (1.0 + (-1.0 * &logits).exp());
        let y_classif = p.mapv(|prob| {
            let bernoulli = Bernoulli::new(prob).unwrap();
            if bernoulli.sample(&mut rnd_seed) {
                1.0
            } else {
                0.0
            }
        });

        let selector =
            OrthogonalSelector::new(vec![0, 3], ScoreType::LogitGradient, 0.05, true, None);

        let (selected, _scores) = selector.fit(&mut x, &y_classif, None).unwrap();

        assert_eq!(selected, vec![0, 3]);
    }
}
