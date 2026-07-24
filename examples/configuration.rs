//! Configuring fixed-point evaluation algorithms.

use fixed_math::prelude::*;

const FRACTIONAL_BITS: u32 = 16;
const DEPTH: usize = 24;
const SCALE: f64 = 65_536.0;

fn to_decimal(value: Fixed) -> f64 {
    value.raw() as f64 / SCALE
}

fn evaluate_exp(input: Fixed, algorithm: EvaluationAlgorithm) -> Result<Fixed> {
    let config = EvaluationConfig::binary(FRACTIONAL_BITS, DEPTH).with_algorithm(algorithm);

    let function = Exp::with_config(input, config);

    Evaluate::evaluate(&function)
}

fn main() -> Result<()> {
    let format = Format::new(FRACTIONAL_BITS);
    let input = Fixed::encode_integer(1, format)?;

    let backward = evaluate_exp(input, EvaluationAlgorithm::Backward)?;

    let lentz = evaluate_exp(input, EvaluationAlgorithm::Lentz)?;

    println!(
        "Backward: {:.6} (raw: {})",
        to_decimal(backward),
        backward.raw(),
    );

    println!("Lentz:    {:.6} (raw: {})", to_decimal(lentz), lentz.raw(),);

    println!("Raw difference: {}", backward.raw().abs_diff(lentz.raw()),);

    Ok(())
}
