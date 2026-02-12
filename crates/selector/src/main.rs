use ndarray::prelude::*;
use ndarray_rand::rand;
use ndarray_rand::rand_distr::{Bernoulli, Distribution};
use polars::prelude::*;
use selector::get_random_normal;

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

    let logits = 2.0 * &z1 + 1.5 * &z2;
    let p = 1.0 / (1.0 + (-1.0 * &logits).exp());

    let y_classif: Array2<i32> = p.mapv(|prob| {
        let bernoulli = Bernoulli::new(prob).unwrap();
        if bernoulli.sample(&mut rnd_seed) {
            1
        } else {
            0
        }
    });

    println!("{}", &y_classif.slice(s![2..5, ..]));
}
