use serde::Serializer;
use rust_decimal::Decimal;

const PRECISION: u32 = 4;

pub fn fixed_precision_serializer<S: Serializer>(x: &Decimal, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&x.round_dp(PRECISION).to_string())
}