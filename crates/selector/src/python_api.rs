use crate::prelude::HashMap;
use numpy::{PyArray1, PyReadonlyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::LogisticRegressionParams;
use crate::distance_correlation::DistanceCorrelation;
use crate::{OrthogonalSelector as RustOrthogonalSelector, ScoreType};

#[pyclass(name = "OrthogonalSelector", module = "selector")]
pub struct PyOrthogonalSelector {
    fixed_feature_indices: Vec<usize>,
    score_type: ScoreType,
    min_score: f32,
    center_features: bool,
    n_jobs: Option<usize>,
    logistic_params: Option<LogisticRegressionParams>,
}

#[pymethods]
impl PyOrthogonalSelector {
    #[new]
    #[pyo3(signature = (fixed_feature_indices, score_type, min_score, center_features, n_jobs=None, *, max_iter=None, alpha=None, learning_rate=None, r_tol=None))]
    fn new(
        fixed_feature_indices: Vec<usize>,
        score_type: &str,
        min_score: f32,
        center_features: bool,
        n_jobs: Option<usize>,
        max_iter: Option<usize>,
        alpha: Option<f32>,
        learning_rate: Option<f32>,
        r_tol: Option<f32>,
    ) -> PyResult<Self> {
        let log_params_default = LogisticRegressionParams::default();
        let logistic_params = Some(LogisticRegressionParams::new(
            max_iter.unwrap_or(log_params_default.max_iter),
            alpha.unwrap_or(log_params_default.alpha),
            learning_rate.unwrap_or(log_params_default.learning_rate),
            r_tol.unwrap_or(log_params_default.r_tol),
        ));

        Ok(Self {
            fixed_feature_indices,
            score_type: parse_score_type(score_type)?,
            min_score,
            center_features,
            n_jobs,
            logistic_params,
        })
    }

    #[pyo3(signature = (x, y, sample_weights=None))]
    fn fit<'py>(
        &self,
        py: Python<'py>,
        x: PyReadonlyArray2<'py, f32>,
        y: PyReadonlyArray2<'py, f32>,
        sample_weights: Option<PyReadonlyArray2<'py, f32>>,
    ) -> PyResult<(Vec<usize>, HashMap<usize, f32>)> {
        let data = x.as_array();
        let y = y.as_array();
        let sample_weights = sample_weights.as_ref().map_or(None, |a| Some(a.as_array()));

        validate_shape_one_col(y.shape(), "y")?;
        validate_row_count(data.nrows(), y.nrows(), "y")?;

        if let Some(ref weights) = sample_weights {
            validate_shape_one_col(weights.shape(), "sample weights")?;
            validate_row_count(data.nrows(), weights.nrows(), "sample_weights")?;
        }

        // clone of parameters is needed because of detach
        let fixed_feature_indices = self.fixed_feature_indices.clone();
        let score_type = self.score_type;
        let min_score = self.min_score;
        let center_features = self.center_features;
        let n_jobs = self.n_jobs;
        let logistic_regression_params = self.logistic_params;

        let result = py.detach(move || {
            let selector = RustOrthogonalSelector::new(
                fixed_feature_indices,
                score_type,
                min_score,
                center_features,
                n_jobs,
                logistic_regression_params,
            );

            selector
                .fit(data, y, sample_weights)
                .map_err(|err| err.to_string())
        });

        result.map_err(PyValueError::new_err)
    }

    fn __repr__(&self) -> String {
        let logistic_params = self.logistic_params.unwrap_or_default();

        format!(
            "OrthogonalSelector(fixed_feature_indices={:?}, score_type='{}', min_score={}, center_features={}, n_jobs={:?}, max_iter={}, alpha={}, learning_rate={}, r_tol={})",
            self.fixed_feature_indices,
            score_type_name(self.score_type),
            self.min_score,
            self.center_features,
            self.n_jobs,
            logistic_params.max_iter,
            logistic_params.alpha,
            logistic_params.learning_rate,
            logistic_params.r_tol,
        )
    }
}

#[pyfunction(signature = (x, y, n_jobs=1, sample_size=None))]
fn distance_correlation<'py>(
    py: Python<'py>,
    x: PyReadonlyArray2<'py, f32>,
    y: PyReadonlyArray2<'py, f32>,
    n_jobs: Option<usize>,
    sample_size: Option<usize>,
) -> PyResult<Bound<'py, PyArray1<f32>>> {
    let data = x.as_array();
    let y = y.as_array();

    validate_shape_one_col(y.shape(), "y")?;
    validate_row_count(data.nrows(), y.nrows(), "y")?;

    let dcor = DistanceCorrelation::new(n_jobs.unwrap_or(1), sample_size);
    let scores = py.detach(move || dcor.fit_transform(data, y));
    Ok(PyArray1::from_vec(py, scores))
}

#[pymodule]
fn selector(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyOrthogonalSelector>()?;
    module.add_function(wrap_pyfunction!(distance_correlation, module)?)?;
    Ok(())
}

fn parse_score_type(score_type: &str) -> PyResult<ScoreType> {
    match score_type {
        "residual_variance_ratio" => Ok(ScoreType::ResidualVarianceRatio),
        "squared_partial_correlation" => Ok(ScoreType::SquaredPartialCorrelation),
        "logit_gradient" => Ok(ScoreType::LogitGradient),
        _ => Err(PyValueError::new_err(format!(
            "Unsupported score_type `{score_type}`. Expected one of: residual_variance_ratio, squared_partial_correlation, logit_gradient"
        ))),
    }
}

fn score_type_name(score_type: ScoreType) -> &'static str {
    match score_type {
        ScoreType::ResidualVarianceRatio => "residual_variance_ratio",
        ScoreType::SquaredPartialCorrelation => "squared_partial_correlation",
        ScoreType::LogitGradient => "logit_gradient",
    }
}

fn validate_row_count(expected_rows: usize, actual_rows: usize, name: &str) -> PyResult<()> {
    if expected_rows == actual_rows {
        return Ok(());
    }

    Err(PyValueError::new_err(format!(
        "`{name}` must have {expected_rows} rows to match the feature matrix; got {actual_rows}"
    )))
}

fn validate_shape_one_col(shape: &[usize], name: &str) -> PyResult<()> {
    if shape.len() == 2 && shape[1] == 1 {
        return Ok(());
    }

    Err(PyValueError::new_err(format!(
        "`{name}` must be a 2D column vector with shape (n_rows, 1); got {:?}",
        shape
    )))
}
