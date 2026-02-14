use ndarray::prelude::*;
use ndarray_rand::rand_distr::{Bernoulli, Distribution};
use ndarray_rand::{RandomExt, rand};
use polars::prelude::*;
use selector::*;

pub fn pl() -> Result<(), PolarsError> {
    // let file = std::fs::File::open("some_data.csv")?;
    let mut df = LazyCsvReader::new("some_data.csv".into()).finish()?;

    println!("{}", df.clone().collect()?);

    // let df2 = df.with_column((&df["col1"] + 2).with_name("new_col".into()))?;
    df = df.with_columns([(col("col1") + lit(2)).alias("new_col")]);

    println!("{}", df.collect()?);

    Ok(())
}

pub fn main() {
    // let a = Normal::new(mean, std_dev);
    let mut rnd_seed = rand::rng();
    let N = 5000;

    let z1 = get_random_normal(N);
    let z2 = get_random_normal(N);

    let x1 = &z1 + 0.05 * get_random_normal(N);
    let x2 = 1.1 * &z1 + 0.05 * get_random_normal(N);
    let x3 = -0.75 * &z1 + 0.05 * get_random_normal(N);

    let x4 = &z2 + 0.05 * get_random_normal(N);
    let x5 = get_random_normal(N);

    let logits = 2.0 * &z1 + 1.5 * &z2 + 0.1;
    let p = 1.0 / (1.0 + (-1.0 * &logits).exp());
    let w = Array::ones((N, 1));

    let y_classif: Array2<i32> = p.mapv(|prob| {
        let bernoulli = Bernoulli::new(prob).unwrap();
        if bernoulli.sample(&mut rnd_seed) {
            1
        } else {
            0
        }
    });

    println!("{}", &y_classif.slice(s![0..15, ..]));

    let w_inner = weighted_inner(&z1, &logits, &w);
    println!("WI {}", w_inner);

    let norm = weighted_norm(&z1, &w);
    println!("Norm {}", norm);

    let X = ndarray::concatenate(
        Axis(1),
        // &[z1.view(), z2.view()]
        &[x1.view(), x2.view(), x3.view(), x4.view(), x5.view()],
    )
    .unwrap();

    // let mut model = LogisticRegression::default();
    let mut model = LogisticRegression::new(1000, 2.0, 0.05, 1e-4);
    let mut scaler = StandardScaler::new();

    let x_std = scaler.fit_transform(&X).unwrap();
    println!("fit");
    let _ = model.fit(&x_std, &y_classif, Some(w)).unwrap();
    println!("after fit");
    let pred = model.predict(&x_std).unwrap();

    // println!("{}", pred.sample_axis(Axis(0), 10, ndarray_rand::SamplingStrategy::WithoutReplacement));
    println!("{}", &pred.slice(s![0..15, ..]));

    let c = model.coeff.unwrap();
    let i = model.intercept.unwrap();
    let loss = model.losses;

    println!("Coeffs: {}", c);
    println!("intercept: {i}");
    // println!("Losses: {:?}, {}", loss, loss.len());
    println!("N Iter: {}", loss.len());

    let correct = (&y_classif.mapv(|f| f as f64) - &pred)
        .mapv(|x| x.abs() < 1e-12)
        .mapv(|b| b as usize)
        .sum();

    let accuracy = correct as f64 / y_classif.len() as f64;
    println!("acc: {accuracy}");
}
