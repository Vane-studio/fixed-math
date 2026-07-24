//! Basic usage of fixed-math.

use fixed_math::prelude::*;

const FRACTIONAL_BITS: u32 = 16;
const SCALE: f64 = 65_536.0;

fn to_decimal(value: Fixed) -> f64 {
    value.raw() as f64 / SCALE
}

fn main() -> Result<()> {
    let format = Format::new(FRACTIONAL_BITS);

    let one = Fixed::encode_integer(1, format)?;
    let two = Fixed::encode_integer(2, format)?;

    let exponential = Evaluate::evaluate(&Exp::new(one))?;
    let logarithm = Evaluate::evaluate(&Ln::new(two))?;

    println!(
        "exp(1) = {:.6} (raw: {})",
        to_decimal(exponential),
        exponential.raw(),
    );

    println!(
        "ln(2)  = {:.6} (raw: {})",
        to_decimal(logarithm),
        logarithm.raw(),
    );

    Ok(())
}
